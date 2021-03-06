use crate::input::{Keyboard, Mouse};
use crate::res::{ResourceLoaderManager, ResourceParser};
use backend::*;
use nae_core::*;
use nae_core::{BaseSystem, BuilderOpts, Event};
use std::cell::RefCell;
use std::cell::RefMut;
use std::collections::VecDeque;
use std::rc::Rc;

//TODO backend requirements for resvg https://github.com/RazrFalcon/resvg/blob/master/docs/backend_requirements.md

pub struct App {
    resources: ResourceLoaderManager,
    sys: System,
    fps: VecDeque<f64>,
    last_time: u64,

    pub delta: f32,
    pub mouse: Mouse,
    pub keyboard: Keyboard,
    pub time: f32,
}

impl BaseApp for App {
    type System = System;

    fn system(&mut self) -> &mut Self::System {
        &mut self.sys
    }
}

impl App {
    pub fn load_resource<T>(&mut self, file: &str) -> Result<T, String>
    where
        T: Resource<Self> + ResourceParser<App = App> + 'static,
    {
        let res = T::prepare(self, file)?;
        self.resources.add(file, Box::new(res.clone()))?;
        Ok(res)
    }

    pub fn gfx(&mut self) -> &mut Graphics {
        &mut self.sys.draw().gfx
    }

    pub fn draw(&mut self) -> &mut Draw {
        self.sys.draw()
    }

    fn tick(&mut self) {
        let now = date_now();
        let elapsed = (now - self.last_time) as f64;
        self.last_time = now;
        self.delta = (elapsed / 1000.0) as f32;
        self.time += self.delta;
        self.fps.pop_front();
        self.fps.push_back(elapsed);
    }

    pub fn fps(&self) -> f64 {
        let average: f64 = self.fps.iter().sum::<f64>() / self.fps.len() as f64;
        1000.0 / average
    }

    pub fn width(&self) -> f32 {
        self.sys.width()
    }

    pub fn height(&self) -> f32 {
        self.sys.height()
    }

    pub fn dpi(&self) -> f32 {
        self.sys.dpi()
    }

    pub fn set_fullscreen(&mut self, full: bool) {
        self.system().set_fullscreen(full);
    }

    pub fn fullscreen(&mut self) -> bool {
        self.system().fullscreen()
    }
}

pub struct AppBuilder<S>
where
    S: 'static,
{
    state_cb: fn(&mut App) -> S,
    draw_callback: Option<fn(&mut App, &mut S)>,
    update_callback: Option<fn(&mut App, &mut S)>,
    start_callback: Option<fn(&mut App, &mut S)>,
    event_callback: Option<fn(&mut App, &mut S, event: Event)>,
    options: BuilderOpts,
}

impl<S> AppBuilder<S> {
    pub fn build(&mut self) -> Result<(), String> {
        let sys = System::new(self.options.clone())?;
        let width = sys.width();
        let height = sys.height();

        let mut fps = VecDeque::with_capacity(300);
        fps.resize(fps.capacity(), 1000.0 / 60.0);

        let mut app = App {
            sys: sys,
            resources: ResourceLoaderManager::new(),
            fps: fps,
            last_time: date_now(),
            delta: 0.0,
            time: 0.0,
            mouse: Mouse::new(),
            keyboard: Keyboard::new(),
        };

        let dpi = app.dpi();
        app.draw().update_dpi(dpi);

        let mut state = (self.state_cb)(&mut app);
        let draw_cb = self.draw_callback.take().unwrap_or(|_, _| {});
        let update_cb = self.update_callback.take().unwrap_or(|_, _| {});
        let start_cb = self.start_callback.take().unwrap_or(|_, _| {});
        let event_cb = self.event_callback.take().unwrap_or(|_, _, _| {});

        start_cb(&mut app, &mut state);
        backend::run(
            app,
            state,
            move |mut app, mut state| {
                app.tick();
                try_load_resources(&mut app).unwrap();
                process_events(app, state, event_cb);
                update_cb(&mut app, &mut state);
            },
            move |mut app, mut state| {
                draw_cb(&mut app, &mut state);
            },
        )?;

        Ok(())
    }

    pub fn keep_window_aspect(&mut self) -> &mut Self {
        //TODO just for wasm32, if it's enabled keep the canvas apsect/ratio when resizable or maximized is enabled
        unimplemented!()
    }

    pub fn size(&mut self, width: i32, height: i32) -> &mut Self {
        self.options.width = width;
        self.options.height = height;
        self
    }

    pub fn min_size(&mut self, width: i32, height: i32) -> &mut Self {
        self.options.min_size = Some((width, height));
        self
    }

    pub fn max_size(&mut self, width: i32, height: i32) -> &mut Self {
        self.options.max_size = Some((width, height));
        self
    }

    pub fn title(&mut self, title: &str) -> &mut Self {
        self.options.title = title.to_string();
        self
    }

    pub fn fullscreen(&mut self) -> &mut Self {
        self.options.fullscreen = true;
        self
    }

    pub fn maximized(&mut self) -> &mut Self {
        self.options.maximized = true;
        self
    }

    pub fn icon(&mut self) -> &mut Self {
        //TODO set window icon
        unimplemented!()
    }

    pub fn resizable(&mut self) -> &mut Self {
        self.options.resizable = true;
        self
    }

    pub fn draw(&mut self, cb: fn(&mut App, &mut S)) -> &mut Self {
        self.draw_callback = Some(cb);
        self
    }

    pub fn start(&mut self, cb: fn(&mut App, &mut S)) -> &mut Self {
        self.start_callback = Some(cb);
        self
    }

    pub fn update(&mut self, cb: fn(&mut App, &mut S)) -> &mut Self {
        self.update_callback = Some(cb);
        self
    }

    pub fn event(&mut self, cb: fn(&mut App, &mut S, event: Event)) -> &mut Self {
        self.event_callback = Some(cb);
        self
    }

    pub fn fps_target(&mut self, fps: i32) -> &mut Self {
        //TODO implement this on the backends
        self.options.fps_target = Some(fps);
        self
    }
}

fn process_events<S>(app: &mut App, state: &mut S, cb: fn(&mut App, &mut S, Event)) {
    app.mouse.clear();
    app.keyboard.clear();
    let mut events = app.sys.events().take_events();
    for evt in events {
        app.mouse.process(&evt, app.delta);
        app.keyboard.process(&evt, app.delta);

        match evt {
            Event::WindowResize { width, height } => {
                app.sys.draw().set_size(width as _, height as _);
            }
            Event::ScreenAspectChange { ratio } => {
                app.sys.draw().update_dpi(ratio);
            }
            _ => {}
        }

        cb(app, state, evt);
    }
}

//TODO don't stop the loop, just return Vec<String> with the errors, and the user will decide what to do instead of stop the program
fn try_load_resources(app: &mut App) -> Result<(), String> {
    if let Some(mut assets_loaded) = app.resources.try_load()? {
        while let Some((data, mut asset)) = assets_loaded.pop() {
            asset.parse_resource(app, data)?;
        }
    }

    Ok(())
}

pub fn init() -> AppBuilder<()> {
    init_with(|_| ())
}

pub fn init_with<S>(cb: fn(&mut App) -> S) -> AppBuilder<S> {
    AppBuilder {
        state_cb: cb,
        draw_callback: None,
        update_callback: None,
        start_callback: None,
        event_callback: None,
        options: BuilderOpts::default(),
    }
}

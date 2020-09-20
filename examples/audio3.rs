use nae::prelude::*;

struct State {
    ctx: AudioContext,
    audio: AudioSource,
    snd: Option<Audio>,
    fnt: Font,
}

#[nae::main]
fn main() {
    nae::init_with(|app| State {
        fnt: Font::from_bytes(app, include_bytes!("./assets/Ubuntu-B.ttf")).unwrap(),
        ctx: AudioContext::new().unwrap(),
        audio: AudioSource::from_bytes(app, include_bytes!("./assets/engine3.ogg")).unwrap(),
        snd: None,
    })
    .draw(draw)
    .build()
    .unwrap();
}

fn draw(app: &mut App, state: &mut State) {
    state.ctx.tick();

    if app.keyboard.was_pressed(KeyCode::Space) {
        if state.snd.is_none() {
            state.snd = Some(state.ctx.instance(&state.audio));
        }

        if let Some(snd) = &mut state.snd {
            snd.toggle_play();
        }
    }

    if let Some(snd) = &mut state.snd {
        if app.keyboard.is_down(KeyCode::Q) {
            snd.set_volume(snd.volume() - app.delta);
        }

        if app.keyboard.is_down(KeyCode::A) {
            snd.set_volume(snd.volume() + app.delta);
        }
    }

    if app.keyboard.is_down(KeyCode::W) {
        state.ctx.set_volume(state.ctx.volume() - app.delta);
    }

    if app.keyboard.is_down(KeyCode::S) {
        state.ctx.set_volume(state.ctx.volume() + app.delta);
    }

    let draw = app.draw();
    draw.begin(Color::new(0.1, 0.2, 0.3, 1.0));
    draw.align_text_to(TextAlign::Center);
    draw.text(
        &state.fnt,
        "Press SPACE to start the audio",
        400.0,
        300.0,
        40.0,
    );
    draw.text(
        &state.fnt,
        &format!(
            "Audio Volume: {}",
            state.snd.as_ref().map_or(1.0, |snd| snd.volume())
        ),
        400.0,
        380.0,
        20.0,
    );
    draw.text(
        &state.fnt,
        &format!("Global Volume: {}", state.ctx.volume()),
        400.0,
        450.0,
        20.0,
    );
    draw.end();
}

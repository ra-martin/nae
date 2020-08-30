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
    if app.keyboard.was_pressed(KeyCode::Space) {
        if state.snd.is_none() {
            state.snd = state.ctx.instance(&state.audio).ok();
        }

        if let Some(snd) = &mut state.snd {
            if snd.is_playing() {
                snd.stop();
            } else {
                snd.play();
            }
        }
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
    draw.end();
}

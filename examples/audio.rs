use nae::prelude::*;

struct State {
    ctx: AudioContext,
    audio: AudioSource,
    fnt: Font,
}

#[nae::main]
fn main() {
    nae::init_with(|app| State {
        fnt: Font::from_bytes(app, include_bytes!("./assets/Ubuntu-B.ttf")).unwrap(),
        ctx: AudioContext::new().unwrap(),
        audio: AudioSource::from_bytes(app, include_bytes!("./assets/engine3.ogg")).unwrap(),
    })
    .draw(draw)
    .build()
    .unwrap();
}

fn draw(app: &mut App, state: &mut State) {
    state.ctx.tick();

    if app.keyboard.was_pressed(KeyCode::Space) {
        state.ctx.play(&state.audio);
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

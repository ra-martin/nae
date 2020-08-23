use nae::prelude::*;

struct State {
    ctx: AudioContext,
    snd1: Audio,
    snd2: Audio,
    dt: f32
}

#[nae::main]
fn main() {
    nae::init_with(|app| State {
        ctx: AudioContext::new().unwrap(),
        snd1: Audio::from_bytes(app, include_bytes!("./assets/engine3.ogg")).unwrap(),
        snd2: Audio::from_bytes(app, include_bytes!("./assets/fireEffect.mp3")).unwrap(),
        dt: 0.0
    })
        .draw(draw)
        .build()
        .unwrap();
}

fn draw(app: &mut App, state: &mut State) {
    state.dt += app.delta;

    if app.keyboard.was_pressed(KeyCode::Space) {
        state.ctx.play(&mut state.snd1);
    }
    if app.keyboard.was_pressed(KeyCode::A) {
        state.ctx.stop(&mut state.snd1);
    }

    let draw = app.draw();
    draw.begin(Color::new(0.1, 0.2, 0.3, 1.0));
    draw.color = Color::GREEN;
    draw.push_rotation_from(state.dt, 400.0, 300.0);
    draw.triangle(400.0, 100.0, 100.0, 500.0, 700.0, 500.0);
    draw.pop();
    draw.end();
}

use nae::prelude::*;

struct State {
    tex: Texture,
    count: f32,
}

#[nae::main]
fn main() {
    nae::init_with(init).draw(draw).build().unwrap();
}

fn draw(app: &mut App, state: &mut State) {
    let cos = state.count.cos();
    let sin = state.count.sin();
    let img = &state.tex;

    let initial_sx = 2.0;
    let initial_sy = 2.0;

    let sx = initial_sx + cos * 0.5;
    let sy = initial_sy + sin * 0.5;

    let draw = app.draw();
    draw.begin(Color::new(0.1, 0.2, 0.3, 1.0));

    draw.push_scale(sx, sy);
    draw.image(img, 0.0, 0.0);
    draw.pop();

    draw.end();

    state.count += 0.005;
}

fn init(app: &mut App) -> State {
    State {
        tex: Texture::from_bytes(app, include_bytes!("assets/ferris.png")).unwrap(),
        count: 0.0,
    }
}

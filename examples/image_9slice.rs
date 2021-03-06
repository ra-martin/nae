use nae::prelude::*;

fn init(app: &mut App) -> Texture {
    Texture::from_bytes(app, include_bytes!("assets/green_panel.png")).unwrap()
}

fn draw(app: &mut App, panel: &mut Texture) {
    let draw = app.draw();
    draw.begin(Color::WHITE);
    draw.image(panel, 10.0, 10.0);
    draw.image_9slice(panel, 200.0, 120.0, 400.0, 300.0);
    draw.end();
}

#[nae::main]
fn main() {
    nae::init_with(init).draw(draw).build().unwrap();
}

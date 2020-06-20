use nae::prelude::*;
use nae::ui::{BorderStyle, CornerRadius, UIButton, UIButtonStyle, UIContext};

struct State {
    ctx: Option<UIContext<Self>>,
}

#[nae::main]
fn main() {
    nae::init_with(init).draw(draw).build().unwrap();
}

fn init(app: &mut App) -> State {
    let mut ctx = UIContext::new();
    let mut btn = UIButton::new(200.0, 100.0);
    let mut style = UIButtonStyle::default();
    style.background_color = Some(Color::RED);
    style.border = Some(BorderStyle {
        width: 0.0,
        corner_radius: CornerRadius::Radius(0.0),
        color: None,
    });

    btn.style = style;

    ctx.elements.push(Box::new(btn));

    State { ctx: Some(ctx) }
}

fn draw(app: &mut App, state: &mut State) {
    // let draw = app.draw();
    // draw.begin(Color::new(0.1, 0.2, 0.3, 1.0));
    if let Some(mut ctx) = state.ctx.take() {
        let draw = app.draw();
        draw.begin(Color::ORANGE);
        ctx.draw(draw, state);
        draw.end();

        state.ctx = Some(ctx);
    }

    // draw.end();
}

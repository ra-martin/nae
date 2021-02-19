use nae::prelude::*;

struct State {
    pipeline: Pipeline,
    u_delta_loc: Uniform,
    u_size_loc: Uniform,
    u_brush_loc: Uniform,
    count: f32,
    backbuffer_a: RenderTarget,
    backbuffer_b: RenderTarget
}

#[nae::main]
fn main() {
    nae::init_with(init).draw(draw).build().unwrap();
}

fn init(app: &mut App) -> State {

    let pipeline =
        Pipeline::from_image_fragment(app.gfx(), include_bytes!("assets/shaders/reaction_diffuse.frag.spv"))
            .unwrap();

    let u_delta_loc = pipeline.uniform_location("u_delta").unwrap();
    let u_size_loc = pipeline.uniform_location("u_size").unwrap();
    let u_brush_loc = pipeline.uniform_location("u_brush").unwrap();

    let backbuffer_a = RenderTarget::from_size(app, app.width() as _, app.height() as _, false).unwrap();
    let backbuffer_b = RenderTarget::from_size(app, app.width() as _, app.height() as _, false).unwrap();

    State {
        pipeline,
        u_delta_loc,
        u_size_loc,
        u_brush_loc,
        count: 0.0,
        backbuffer_a,
        backbuffer_b
    }
}

fn draw(app: &mut App, state: &mut State) {

    let width = app.width();
    let height = app.height();
    let center_w = width * 0.5;
    let center_h = height * 0.5;

    let mouse_x = 0.5 * ((app.mouse.x / width) - 0.5);
    let mouse_y = 0.5 * ((app.mouse.y / height) - 0.5);
    let mouse_down = app.mouse.is_down(MouseButton::Left);

    let delta = app.delta * 50.;

    let draw = app.draw();

    // We always draw from backbuffer_a to backbuffer_b
    // and then swap them after
    draw.gfx.set_render_target(Some(&state.backbuffer_b));
    if !(state.count > 0.0) {

        // In our first frame, draw some seed-image
        draw.begin(Color::RED);

        draw.color = Color::GREEN;
        draw.circle(center_w, center_h, 50.);
        
        draw.color = Color::RED;
        draw.circle(center_w, center_h, 25.);

        draw.end();

    } else {

        draw.begin(Color::WHITE);

        draw.blend_mode = BlendMode::NONE;
        draw.color = Color::WHITE;
        
        draw.set_pipeline(Some(&state.pipeline));
        draw.set_uniform(&state.u_delta_loc, &delta);
        draw.set_uniform(&state.u_size_loc, &[width, height]);
        
        if mouse_down {
            draw.set_uniform(&state.u_brush_loc, &[mouse_x, mouse_y]);
        } else {
            draw.set_uniform(&state.u_brush_loc, &[-10., -10.]);
        }
            
        draw.image(&state.backbuffer_a.texture, 0.0, 0.0);
        draw.set_pipeline(None);
        
        draw.end();
    }
    
    draw.gfx.set_render_target(None);

    draw.begin(Color::WHITE);

    draw.blend_mode = BlendMode::NONE;
    draw.color = Color::WHITE;

    draw.image(&state.backbuffer_b.texture, 0.0, 0.0);
    draw.end();

    std::mem::swap(&mut state.backbuffer_a, &mut state.backbuffer_b);

    state.count += app.delta;
}

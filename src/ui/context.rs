use crate::m2d::Transform2d;
use crate::App;
use backend::Draw;
use nae_core::Color;
use nae_gfx::Matrix4;

pub struct UIContext<T> {
    pub elements: Vec<Box<dyn UIElement<T>>>,
}

impl<T> UIContext<T> {
    pub fn new() -> Self {
        Self { elements: vec![] }
    }

    pub fn set_input_state(&mut self, app: &mut App) {
        // Save mouse position, keyboard, etc...
    }

    pub fn add_element(&mut self, element: Box<dyn UIElement<T>>) {
        self.elements.push(element);
    }

    pub fn draw(&mut self, draw: &mut Draw, state: &mut T) {
        self.elements.iter_mut().for_each(|element| {
            draw.push(element.matrix());
            element.draw(draw, state);
            draw.pop();
        });
    }
}

// something like swift?
pub trait UIElement<T> {
    fn draw(&mut self, draw: &mut Draw, state: &mut T) {}
    fn matrix(&mut self) -> &Matrix4;
}

trait UIBehavior {}

#[derive(Clone, Copy)]
pub enum CornerRadius {
    None,
    Radius(f32),
    Custom {
        top: f32,
        left: f32,
        right: f32,
        bottom: f32,
    },
}

impl Default for CornerRadius {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Default, Clone, Copy)]
pub struct BorderStyle {
    pub width: f32,
    pub corner_radius: CornerRadius,
    pub color: Option<Color>,
}

#[derive(Default, Clone, Copy)]
pub struct UIButtonStyle {
    pub background_color: Option<Color>,
    pub border: Option<BorderStyle>,
}

pub struct UIButton {
    pub width: f32,
    pub height: f32,
    pub style: UIButtonStyle,

    pub transform: Transform2d,
}

impl UIButton {
    pub fn new(width: f32, height: f32) -> Self {
        let transform = Transform2d::new(width, height);
        let style = UIButtonStyle::default();

        Self {
            width,
            height,
            transform,
            style,
        }
    }
}

impl<T> UIElement<T> for UIButton {
    fn draw(&mut self, draw: &mut Draw, state: &mut T) {
        let ww = self.width;
        let hh = self.height;

        draw.color = self.style.background_color.unwrap_or(Color::TRANSPARENT);
        let radius = if let Some(border) = &self.style.border {
            border.corner_radius
        } else {
            CornerRadius::None
        };

        match radius {
            CornerRadius::None => {
                draw.rect(0.0, 0.0, ww, hh);
            }
            CornerRadius::Radius(radius) => {
                if radius == 0.0 {
                    draw.rect(0.0, 0.0, ww, hh);
                } else {
                    draw.rounded_rect(0.0, 0.0, ww, hh, radius);
                }
            }
            CornerRadius::Custom {
                top,
                right,
                left,
                bottom,
            } => {
                //TODO
                //draw.rounded_rect(0.0, 0.0, ww, hh, radius);
            }
        }
    }

    fn matrix(&mut self) -> &Matrix4 {
        &self.transform.matrix()
    }
}

use crate::{BaseApp, BaseGfx, BaseSystem};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TextureFormat {
    Rgba,
    Red,
    R8,
    Depth,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TextureFilter {
    Linear,
    Nearest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextAlign {
    TopLeft,
    TopCenter,
    TopRight,

    MiddleLeft,
    Center,
    MiddleRight,

    BottomLeft,
    BottomCenter,
    BottomRight,
}

impl TextAlign {
    pub fn get_align(&self) -> (VerticalAlign, HorizontalAlign) {
        use TextAlign::*;
        match self {
            TopLeft => (VerticalAlign::Top, HorizontalAlign::Left),
            TopCenter => (VerticalAlign::Top, HorizontalAlign::Center),
            TopRight => (VerticalAlign::Top, HorizontalAlign::Right),

            MiddleLeft => (VerticalAlign::Center, HorizontalAlign::Left),
            Center => (VerticalAlign::Center, HorizontalAlign::Center),
            MiddleRight => (VerticalAlign::Center, HorizontalAlign::Right),

            BottomLeft => (VerticalAlign::Bottom, HorizontalAlign::Left),
            BottomCenter => (VerticalAlign::Bottom, HorizontalAlign::Center),
            BottomRight => (VerticalAlign::Bottom, HorizontalAlign::Right),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HorizontalAlign {
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VerticalAlign {
    Top,
    Center,
    Bottom,
}

/// Represents an external resource
pub trait Resource<T>: Clone {
    /// Create a empty resource ready to be loaded
    fn prepare(app: &mut T, file: &str) -> Result<Self, String>;

    /// Parse byte data to create to fill the resource
    fn set_data(&mut self, app: &mut T, data: Vec<u8>) -> Result<(), String>;
}

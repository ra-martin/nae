/// Represents a visual color
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    /// Red value
    pub r: f32,
    /// Green value
    pub g: f32,
    /// Blue value
    pub b: f32,
    /// Alpha value
    pub a: f32,
}

impl Color {
    pub const TRANSPARENT: Color = Color::new(0.0, 0.0, 0.0, 0.0);
    pub const WHITE: Color = Color::new(1.0, 1.0, 1.0, 1.0);
    pub const BLACK: Color = Color::new(0.0, 0.0, 0.0, 1.0);
    pub const RED: Color = Color::new(1.0, 0.0, 0.0, 1.0);
    pub const GREEN: Color = Color::new(0.0, 1.0, 0.0, 1.0);
    pub const BLUE: Color = Color::new(0.0, 0.0, 1.0, 1.0);
    pub const YELLOW: Color = Color::new(1.0, 1.0, 0.0, 1.0);
    pub const MAGENTA: Color = Color::new(1.0, 0.0, 1.0, 1.0);
    pub const SILVER: Color = Color::new(0.753, 0.753, 0.753, 1.0);
    pub const GRAY: Color = Color::new(0.5, 0.5, 0.5, 1.0);
    pub const OLIVE: Color = Color::new(0.5, 0.5, 0.0, 1.0);
    pub const PURPLE: Color = Color::new(0.5, 0.0, 0.5, 1.0);
    pub const MAROON: Color = Color::new(0.5, 0.0, 0.0, 1.0);
    pub const AQUA: Color = Color::new(0.0, 1.0, 1.0, 1.0);
    pub const TEAL: Color = Color::new(0.0, 0.5, 0.5, 1.0);
    pub const NAVY: Color = Color::new(0.0, 0.0, 0.5, 1.0);
    pub const ORANGE: Color = Color::new(1.0, 0.647, 0.0, 1.0);
    pub const PINK: Color = Color::new(1.0, 0.753, 0.796, 1.0);

    /// Create a new color from red, green, blue and alpha values
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// Create a new color from red, green, blue and alpha values
    pub const fn from_rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self::new(r, g, b, a)
    }

    /// Create a new color from red, green and blue values
    pub const fn from_rgb(r: f32, g: f32, b: f32) -> Self {
        Self::from_rgba(r, g, b, 1.0)
    }

    /// Create a new color from hexadecimal number like 0x000000ff (0xRRGGBBAA)
    pub fn from_hex(hex: u32) -> Self {
        let [r, g, b, a] = hex_to_rgba(hex);
        Self { r, g, b, a }
    }

    /// Create a new color from rgba bytes
    pub fn from_bytes(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: a as f32 / 255.0,
        }
    }

    /// Returns the same color with the red passed
    pub const fn with_red(&self, red: f32) -> Color {
        Self::new(red, self.g, self.b, self.a)
    }

    /// Returns the same color with the green passed
    pub const fn with_green(&self, green: f32) -> Color {
        Self::new(self.r, green, self.b, self.a)
    }

    /// Returns the same color with the blue passed
    pub const fn with_blue(&self, blue: f32) -> Color {
        Self::new(self.r, self.g, blue, self.a)
    }

    /// Returns the same color with the alpha passed
    pub const fn with_alpha(&self, alpha: f32) -> Color {
        Self::new(self.r, self.g, self.b, alpha)
    }

    /// Returns an array with the r, g, b, a values
    pub const fn to_rgba(&self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }

    /// Returns an array with the r, g, b values
    pub const fn to_rgb(&self) -> [f32; 3] {
        [self.r, self.g, self.b]
    }

    /// Returns the hexadecimal representation of the color like 0xRRGGBBAA
    pub fn to_hex(&self) -> u32 {
        rgba_to_hex(self.r, self.g, self.b, self.a)
    }

    /// Returns the hexadecimal representantion of the colos as string like #RRGGBBAA
    pub fn to_hex_string(&self) -> String {
        hex_to_string(self.to_hex())
    }
}

impl From<Color> for [f32; 4] {
    fn from(c: Color) -> Self {
        c.to_rgba()
    }
}

impl From<u32> for Color {
    fn from(color: u32) -> Self {
        Color::from_hex(color)
    }
}

impl From<[f32; 4]> for Color {
    fn from(color: [f32; 4]) -> Self {
        Color::new(color[0], color[1], color[2], color[3])
    }
}

impl From<[f32; 3]> for Color {
    fn from(color: [f32; 3]) -> Self {
        Color::from_rgb(color[0], color[1], color[2])
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Color {{ r: {}, g: {}, b: {}, a: {}}}",
            self.r, self.g, self.b, self.a
        )
    }
}

/// Converts a rgba color values to a hexadecimal values
pub fn rgba_to_hex(r: f32, g: f32, b: f32, a: f32) -> u32 {
    (((r * 255.0) as u32) << 24)
        + (((g * 255.0) as u32) << 16)
        + (((b * 255.0) as u32) << 8)
        + (((a * 255.0) as u32) | 0)
}

/// Converts an hexadecimal value to a rgba values
pub fn hex_to_rgba(hex: u32) -> [f32; 4] {
    [
        ((hex >> 24) & 0xFF) as f32 / 255.0,
        ((hex >> 16) & 0xFF) as f32 / 255.0,
        ((hex >> 8) & 0xFF) as f32 / 255.0,
        (hex & 0xFF) as f32 / 255.0,
    ]
}

/// Converts a hexadecimal value to string
pub fn hex_to_string(hex: u32) -> String {
    format!("{:#X}", hex)
}

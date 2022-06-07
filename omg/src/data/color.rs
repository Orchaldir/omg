/// Represents a color with the RGB color model.
///
/// See [Wikipedia](https://en.wikipedia.org/wiki/RGB_color_model).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    /// Returns a new color.
    pub const fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }

    /// Returns a new gray color.
    ///
    /// ```
    ///# use omg::data::color::Color;
    /// assert_eq!(Color::gray(5), Color::new(5, 5, 5));
    /// ```
    pub const fn gray(value: u8) -> Color {
        Color {
            r: value,
            g: value,
            b: value,
        }
    }

    /// Returns the red component.
    ///
    /// ```
    ///# use omg::data::color::Color;
    /// assert_eq!(Color::new(0, 1, 2).r(), 0);
    /// ```
    pub fn r(&self) -> u8 {
        self.r
    }

    /// Returns the green component.
    ///
    /// ```
    ///# use omg::data::color::Color;
    /// assert_eq!(Color::new(0, 1, 2).g(), 1);
    /// ```
    pub fn g(&self) -> u8 {
        self.g
    }

    /// Returns the blue component.
    ///
    /// ```
    ///# use omg::data::color::Color;
    /// assert_eq!(Color::new(0, 1, 2).b(), 2);
    /// ```
    pub fn b(&self) -> u8 {
        self.b
    }
}

impl Default for Color {
    fn default() -> Self {
        PINK
    }
}

impl From<Color> for [u8; 3] {
    fn from(color: Color) -> Self {
        [color.r, color.g, color.b]
    }
}

impl From<Color> for [f32; 3] {
    fn from(color: Color) -> Self {
        [
            color.r() as f32 / 255.0,
            color.g() as f32 / 255.0,
            color.b() as f32 / 255.0,
        ]
    }
}

pub const BLACK: Color = Color::new(0, 0, 0);
pub const BLUE: Color = Color::new(0, 0, 255);
pub const CYAN: Color = Color::new(0, 255, 255);
pub const GREEN: Color = Color::new(0, 255, 0);
pub const MAGENTA: Color = Color::new(255, 0, 255);
pub const ORANGE: Color = Color::new(255, 165, 0);
pub const RED: Color = Color::new(255, 0, 0);
pub const PINK: Color = Color::new(255, 0, 128);
pub const WHITE: Color = Color::new(255, 255, 255);
pub const YELLOW: Color = Color::new(255, 255, 0);

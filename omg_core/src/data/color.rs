use crate::data::math::interpolation::{lerp, Interpolate};
use anyhow::{bail, Context};

/// Represents a color with the RGB color model.
///
/// See [Wikipedia](https://en.wikipedia.org/wiki/RGB_color_model).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
    ///# use omg_core::data::color::Color;
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
    ///# use omg_core::data::color::Color;
    /// assert_eq!(Color::new(0, 1, 2).r(), 0);
    /// ```
    pub fn r(&self) -> u8 {
        self.r
    }

    /// Returns the green component.
    ///
    /// ```
    ///# use omg_core::data::color::Color;
    /// assert_eq!(Color::new(0, 1, 2).g(), 1);
    /// ```
    pub fn g(&self) -> u8 {
        self.g
    }

    /// Returns the blue component.
    ///
    /// ```
    ///# use omg_core::data::color::Color;
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

/// Converts a color to a hex color code. See <https://en.wikipedia.org/wiki/Web_colors>.
///
/// ```
///# use omg_core::data::color::{Color, ORANGE};
/// let code: String = ORANGE.into();
/// assert_eq!(code, "#FFA500");
/// ```
impl From<Color> for String {
    fn from(color: Color) -> Self {
        format!("#{:02X}{:02X}{:02X}", color.r(), color.g(), color.b())
    }
}

/// Converts a hex color code to a color, if possible:
///
/// ```
///# use omg_core::data::color::{Color, ORANGE};
/// assert_eq!(Color::try_from("#FFA500").unwrap(), ORANGE);
/// ```
impl TryFrom<&str> for Color {
    type Error = anyhow::Error;

    fn try_from(hex_code: &str) -> anyhow::Result<Color> {
        if !hex_code.starts_with('#') {
            bail!("'{}' needs to start with # to be a color", hex_code);
        } else if hex_code.len() != 7 {
            bail!("'{}' needs to be 7 characters long to be a color", hex_code);
        }

        let r: u8 = u8::from_str_radix(&hex_code[1..3], 16)
            .with_context(|| format!("Failed to parse the value of red from '{}'", hex_code))?;
        let g: u8 = u8::from_str_radix(&hex_code[3..5], 16)
            .with_context(|| format!("Failed to parse the value of green from '{}'", hex_code))?;
        let b: u8 = u8::from_str_radix(&hex_code[5..7], 16)
            .with_context(|| format!("Failed to parse the value of blue from '{}'", hex_code))?;

        Ok(Color::new(r, g, b))
    }
}

impl TryFrom<String> for Color {
    type Error = anyhow::Error;

    fn try_from(hex_code: String) -> anyhow::Result<Color> {
        Color::try_from(hex_code.as_str())
    }
}

impl Interpolate for Color {
    /// Interpolates linearly with another color.
    ///
    /// ```
    ///# use omg_core::data::color::Color;
    ///# use omg_core::data::math::interpolation::Interpolate;
    /// let color0 = Color::new(  0, 25, 120);
    /// let color1 = Color::new(200, 75, 220);
    /// let result = Color::new(100, 50, 170);
    ///
    /// assert_eq!(color0.lerp(&color1, 0.5), result);
    /// ```
    fn lerp(&self, other: &Color, factor: f32) -> Color {
        Color {
            r: lerp(self.r, other.r, factor),
            g: lerp(self.g, other.g, factor),
            b: lerp(self.b, other.b, factor),
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_empty_string() {
        assert!(Color::try_from("").is_err());
    }

    #[test]
    fn test_from_string_invalid_start() {
        assert!(Color::try_from("FFA500").is_err());
    }

    #[test]
    fn test_from_string_part() {
        assert!(Color::try_from("#").is_err());
        assert!(Color::try_from("#FF").is_err());
        assert!(Color::try_from("#FFA5").is_err());
        assert!(Color::try_from("#FFA50").is_err());
    }

    #[test]
    fn test_from_string_ignore_case() {
        assert_eq!(Color::try_from("#FFA500").unwrap(), ORANGE);
        assert_eq!(Color::try_from("#ffa500").unwrap(), ORANGE);
    }
}

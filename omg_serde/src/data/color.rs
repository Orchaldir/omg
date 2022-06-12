use omg::data::color::Color;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct ColorSerde {
    r: u8,
    g: u8,
    b: u8,
}

impl From<ColorSerde> for Color {
    fn from(color: ColorSerde) -> Self {
        Color::new(color.r, color.g, color.b)
    }
}

impl From<Color> for ColorSerde {
    fn from(color: Color) -> Self {
        ColorSerde {
            r: color.r(),
            g: color.g(),
            b: color.b(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use omg::data::color::ORANGE;

    #[test]
    fn test_conversion() {
        let color = ORANGE;
        let serde: ColorSerde = color.into();
        let result: Color = serde.into();

        assert_eq!(result, color)
    }
}

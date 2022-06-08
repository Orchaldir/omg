use omg::data::math::size2d::{Size2d, Size2dError};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Size2dSerde {
    width: u32,
    height: u32,
}

impl TryFrom<Size2dSerde> for Size2d {
    type Error = Size2dError;

    fn try_from(value: Size2dSerde) -> Result<Self, Self::Error> {
        Size2d::new(value.width, value.height)
    }
}

impl From<Size2d> for Size2dSerde {
    fn from(value: Size2d) -> Self {
        Size2dSerde {
            width: value.width(),
            height: value.height(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use omg::data::math::size2d::Size2d;
    use omg::data::math::size2d::Size2dError::WidthIsZero;

    #[test]
    fn test_conversion() {
        let start = Size2d::unchecked(1, 2);
        let serde: Size2dSerde = start.into();

        assert_eq!(Size2d::try_from(serde), Ok(start))
    }

    #[test]
    fn test_failed_conversion() {
        let serde = Size2dSerde {
            width: 0,
            height: 1,
        };

        assert_eq!(Size2d::try_from(serde), Err(WidthIsZero))
    }
}

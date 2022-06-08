use omg::data::math::size2d::Size2d;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Size2dSerde {
    width: u32,
    height: u32,
}

impl From<Size2dSerde> for Size2d {
    fn from(value: Size2dSerde) -> Self {
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

    #[test]
    fn test_conversion() {
        let start = Size2d::new(1, 2);
        let serde: Size2dSerde = start.into();
        let result: Size2d = serde.into();

        assert_eq!(result, start)
    }
}

use anyhow::Result;
use omg::data::math::size2d::Size2d;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Size2dSerde {
    width: u32,
    height: u32,
}

impl Size2dSerde {
    pub fn try_convert(&self) -> Result<Size2d> {
        Size2d::new(self.width, self.height)
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
        let start = Size2d::unchecked(1, 2);
        let serde: Size2dSerde = start.into();

        assert_eq!(serde.try_convert().unwrap(), start)
    }

    #[test]
    fn test_failed_conversion() {
        let serde = Size2dSerde {
            width: 0,
            height: 1,
        };

        assert!(serde.try_convert().is_err())
    }
}

use anyhow::Result;
use omg_core::data::math::generator::gradient::Gradient;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct GradientSerde {
    start: u32,
    length: u32,
    value_start: u8,
    value_end: u8,
}

impl GradientSerde {
    pub fn try_convert(&self) -> Result<Gradient> {
        Gradient::new(self.start, self.length, self.value_start, self.value_end)
    }
}

impl From<&Gradient> for GradientSerde {
    fn from(gradient: &Gradient) -> Self {
        GradientSerde {
            start: gradient.start(),
            length: gradient.length(),
            value_start: gradient.value_start(),
            value_end: gradient.value_end(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use omg_core::data::math::generator::gradient::Gradient;

    #[test]
    fn test_conversion() {
        let start = Gradient::new(1000, 100, 100, 200).unwrap();
        let serde: GradientSerde = (&start).into();

        assert_eq!(serde.try_convert().unwrap(), start)
    }
}

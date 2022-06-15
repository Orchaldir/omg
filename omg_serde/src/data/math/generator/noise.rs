use anyhow::Result;
use omg_core::data::math::generator::noise::Noise;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct NoiseSerde {
    seed: u32,
    scale: u32,
    min_value: u8,
    max_value: u8,
}

impl NoiseSerde {
    pub fn try_convert(&self) -> Result<Noise> {
        Noise::new(self.seed, self.scale, self.min_value, self.max_value)
    }
}

impl From<&Noise> for NoiseSerde {
    fn from(noise: &Noise) -> Self {
        NoiseSerde {
            seed: noise.seed(),
            scale: noise.scale(),
            min_value: noise.min_value(),
            max_value: noise.max_value(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use omg_core::data::math::generator::noise::Noise;

    #[test]
    fn test_conversion() {
        let start = Noise::new(1000, 100, 100, 200).unwrap();
        let serde: NoiseSerde = (&start).into();

        assert_eq!(serde.try_convert().unwrap(), start)
    }
}

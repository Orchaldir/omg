use anyhow::Result;
use omg::data::input::Input;
use omg::data::math::interpolation::vector::VectorInterpolator;
use omg::data::math::interpolation::Interpolate;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
struct EntrySerde<T: Input, V: Interpolate> {
    threshold: T,
    value: V,
}

impl<T: Input, V: Interpolate> EntrySerde<T, V> {
    pub fn convert(self) -> (T, V) {
        (self.threshold, self.value)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct VectorInterpolatorSerde<T: Input, V: Interpolate> {
    vector: Vec<EntrySerde<T, V>>,
}

impl<T: Input, V: Interpolate> VectorInterpolatorSerde<T, V> {
    pub fn try_convert(self) -> Result<VectorInterpolator<T, V>> {
        VectorInterpolator::new(
            self.vector
                .into_iter()
                .map(|entry| entry.convert())
                .collect(),
        )
    }
}

impl<T: Input, V: Interpolate> From<&VectorInterpolator<T, V>> for VectorInterpolatorSerde<T, V> {
    fn from(interpolator: &VectorInterpolator<T, V>) -> Self {
        VectorInterpolatorSerde {
            vector: interpolator
                .get_all()
                .iter()
                .map(|entry| EntrySerde {
                    threshold: entry.threshold(),
                    value: entry.value(),
                })
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversion() {
        let start = VectorInterpolator::new(vec![(100u32, 150), (150, 200)]).unwrap();
        let serde: VectorInterpolatorSerde<u32, u8> = (&start).into();

        assert_eq!(serde.try_convert().unwrap(), start)
    }
}

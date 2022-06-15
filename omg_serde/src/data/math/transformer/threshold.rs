use omg_core::data::input::IntInput;
use omg_core::data::math::transformer::threshold::OverwriteWithThreshold;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct OverwriteWithThresholdSerde<T: IntInput> {
    value: T,
    threshold: T,
}

impl<T: IntInput> From<OverwriteWithThresholdSerde<T>> for OverwriteWithThreshold<T> {
    fn from(overwrite: OverwriteWithThresholdSerde<T>) -> Self {
        OverwriteWithThreshold::new(overwrite.value, overwrite.threshold)
    }
}

impl<T: IntInput> From<&OverwriteWithThreshold<T>> for OverwriteWithThresholdSerde<T> {
    fn from(overwrite: &OverwriteWithThreshold<T>) -> Self {
        OverwriteWithThresholdSerde {
            value: overwrite.value(),
            threshold: overwrite.threshold(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversion() {
        let start = OverwriteWithThreshold::new(8, 9);
        let serde: OverwriteWithThresholdSerde<u8> = (&start).into();

        assert_eq!(OverwriteWithThreshold::from(serde), start);
    }
}

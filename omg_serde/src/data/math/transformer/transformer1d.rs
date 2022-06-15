use crate::data::math::transformer::threshold::OverwriteWithThresholdSerde;
use anyhow::Result;
use omg_core::data::input::IntInput;
use omg_core::data::math::transformer::transformer1d::Transformer1d;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub enum Transformer1dSerde<T: IntInput> {
    OverwriteIfAbove(OverwriteWithThresholdSerde<T>),
    OverwriteIfBelow(OverwriteWithThresholdSerde<T>),
    OverwriteWithMap(HashMap<T, T>),
}

type S<T> = Transformer1dSerde<T>;
type R<T> = Transformer1d<T>;

impl<T: IntInput> Transformer1dSerde<T> {
    pub fn try_convert(self) -> Result<Transformer1d<T>> {
        match self {
            S::OverwriteIfAbove(overwrite) => Ok(R::OverwriteIfAbove(overwrite.into())),
            S::OverwriteIfBelow(overwrite) => Ok(R::OverwriteIfBelow(overwrite.into())),
            S::OverwriteWithMap(hashmap) => R::with_map(hashmap),
        }
    }
}

impl<T: IntInput> From<&Transformer1d<T>> for Transformer1dSerde<T> {
    fn from(transformer: &Transformer1d<T>) -> Self {
        match transformer {
            R::OverwriteIfAbove(overwrite) => S::OverwriteIfAbove(overwrite.into()),
            R::OverwriteIfBelow(overwrite) => S::OverwriteIfBelow(overwrite.into()),
            R::OverwriteWithMap(hashmap) => S::OverwriteWithMap(hashmap.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use omg_core::data::math::transformer::threshold::OverwriteWithThreshold;

    #[test]
    fn test_convert_overwrite() {
        let overwrite = OverwriteWithThreshold::new(42, 100);

        assert_eq(Transformer1d::OverwriteIfAbove(overwrite));
        assert_eq(Transformer1d::OverwriteIfBelow(overwrite));
    }

    #[test]
    fn test_convert_overwrite_with_map() {
        let hashmap = vec![(1u8, 25u8), (3, 100)].into_iter().collect();

        assert_eq(Transformer1d::OverwriteWithMap(hashmap));
    }

    fn assert_eq(transformer: Transformer1d<u8>) {
        let serde: Transformer1dSerde<u8> = (&transformer).into();

        assert_eq!(serde.try_convert().unwrap(), transformer)
    }
}

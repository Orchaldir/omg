use crate::data::math::transformer::lookup2d::LookupTable2dSerde;
use crate::data::math::transformer::threshold::OverwriteWithThresholdSerde;
use anyhow::{Context, Result};
use omg_core::data::math::transformer::transformer2d::Transformer2d;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Transformer2dSerde {
    /// Uses both inputs to lookup av alue. E.g. biome from rainfall & temperature.
    Lookup2d(LookupTable2dSerde),
    /// Returns a const value.
    Const(u8),
    /// Overwrites the input, if it is above a threshold.
    OverwriteIfAbove(OverwriteWithThresholdSerde<u8>),
    /// Overwrites the input, if it is below a threshold.
    OverwriteIfBelow(OverwriteWithThresholdSerde<u8>),
}

type S = Transformer2dSerde;
type R = Transformer2d;

impl Transformer2dSerde {
    pub fn try_convert(self) -> Result<Transformer2d> {
        match self {
            S::Lookup2d(lookup) => {
                let lookup_table2d = lookup
                    .try_convert()
                    .context("Failed to convert to Transformer2d::Lookup2d!")?;
                Ok(R::Lookup2d(lookup_table2d))
            }
            S::Const(value) => Ok(R::Const(value)),
            S::OverwriteIfAbove(overwrite) => Ok(R::OverwriteIfAbove(overwrite.into())),
            S::OverwriteIfBelow(overwrite) => Ok(R::OverwriteIfBelow(overwrite.into())),
        }
    }
}

impl From<&Transformer2d> for Transformer2dSerde {
    fn from(transformer: &Transformer2d) -> Self {
        match transformer {
            R::Lookup2d(lookup) => S::Lookup2d(lookup.into()),
            R::Const(value) => S::Const(*value),
            R::OverwriteIfAbove(overwrite) => S::OverwriteIfAbove(overwrite.into()),
            R::OverwriteIfBelow(overwrite) => S::OverwriteIfBelow(overwrite.into()),
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

        assert_eq(Transformer2d::OverwriteIfAbove(overwrite));
        assert_eq(Transformer2d::OverwriteIfBelow(overwrite));
    }

    fn assert_eq(transformer: Transformer2d) {
        let serde: Transformer2dSerde = (&transformer).into();

        assert_eq!(serde.try_convert().unwrap(), transformer)
    }
}

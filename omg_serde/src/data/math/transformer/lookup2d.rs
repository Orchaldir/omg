use crate::data::math::size2d::Size2dSerde;
use anyhow::{Context, Result};
use omg_core::data::math::transformer::lookup2d::LookupTable2d;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LookupTable2dSerde {
    size: Size2dSerde,
    values: Vec<u8>,
}

impl LookupTable2dSerde {
    pub fn try_convert(self) -> Result<LookupTable2d> {
        let size = self
            .size
            .try_convert()
            .context("Failed to convert to LookupTable2d!")?;
        LookupTable2d::new(size, self.values)
    }
}

impl From<&LookupTable2d> for LookupTable2dSerde {
    fn from(lookup: &LookupTable2d) -> Self {
        LookupTable2dSerde {
            size: lookup.size().into(),
            values: lookup.values().into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use omg_core::data::math::size2d::Size2d;

    #[test]
    fn test_conversion() {
        let start = LookupTable2d::new(Size2d::unchecked(2, 1), vec![8, 9]).unwrap();
        let serde: LookupTable2dSerde = (&start).into();

        assert_eq!(serde.try_convert().unwrap(), start)
    }
}

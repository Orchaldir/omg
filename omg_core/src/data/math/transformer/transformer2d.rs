use crate::data::math::transformer::lookup2d::LookupTable2d;
use crate::data::math::transformer::threshold::OverwriteWithThreshold;
use Transformer2d::*;

/// Transforms 2 inputs into an output.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Transformer2d {
    /// Uses both inputs to lookup a value. E.g. biome from rainfall & temperature.
    Lookup2d(LookupTable2d),
    /// Returns a const value.
    Const(u8),
    /// Overwrites the input, if it is above a threshold.
    OverwriteIfAbove(OverwriteWithThreshold<u8>),
    /// Overwrites the input, if it is below a threshold.
    OverwriteIfBelow(OverwriteWithThreshold<u8>),
}

impl Transformer2d {
    pub fn new_overwrite_if_above(value: u8, threshold: u8) -> Transformer2d {
        OverwriteIfAbove(OverwriteWithThreshold::new(value, threshold))
    }

    pub fn new_overwrite_if_below(value: u8, threshold: u8) -> Transformer2d {
        OverwriteIfBelow(OverwriteWithThreshold::new(value, threshold))
    }

    /// Transforms 2 inputs into an output.
    pub fn transform(&self, input0: u8, input1: u8) -> u8 {
        match self {
            Lookup2d(table) => table.lookup(input0, input1),
            Const(value) => *value,
            OverwriteIfAbove(data) => data.overwrite_output_if_above(input0, input1),
            OverwriteIfBelow(data) => data.overwrite_output_if_below(input0, input1),
        }
    }
}

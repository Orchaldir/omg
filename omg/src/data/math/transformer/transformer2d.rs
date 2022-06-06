use crate::data::math::transformer::clusterer2d::{Clusterer2d, Clusterer2dError};
use crate::data::math::transformer::threshold::OverwriteWithThreshold;
use Transformer2d::*;

#[derive(Debug, Eq, PartialEq)]
pub enum Transformer2dError {
    Clusterer(Clusterer2dError),
}

impl From<Clusterer2dError> for Transformer2dError {
    fn from(error: Clusterer2dError) -> Self {
        Transformer2dError::Clusterer(error)
    }
}

/// Transforms 2 inputs into an output.
#[derive(Debug)]
pub enum Transformer2d {
    /// Determine a cluster id from both inputs. E.g. biome from rainfall & temperature.
    Clusterer(Clusterer2d),
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
            Clusterer(clusterer) => clusterer.cluster(input0, input1),
            Const(value) => *value,
            OverwriteIfAbove(data) => data.overwrite_output_if_above(input0, input1),
            OverwriteIfBelow(data) => data.overwrite_output_if_below(input0, input1),
        }
    }
}

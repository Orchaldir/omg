use crate::data::math::interpolation::vector::VectorInterpolatorSerde;
use anyhow::Result;
use omg::data::input::IntInput;
use omg::data::math::interpolation::Interpolate;
use omg::data::math::selector::Selector;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub enum SelectorSerde<I: IntInput, V: Interpolate> {
    Const(V),
    InterpolatePair { first: V, second: V },
    InterpolateVector(VectorInterpolatorSerde<I, V>),
    Lookup { lookup: HashMap<I, V>, default: V },
}

type S<I, V> = SelectorSerde<I, V>;
type R<I, V> = Selector<I, V>;

impl<T: IntInput, V: Interpolate> SelectorSerde<T, V> {
    pub fn try_convert(self) -> Result<Selector<T, V>> {
        match self {
            S::Const(value) => Ok(R::Const(value)),
            S::InterpolatePair { first, second } => Ok(R::InterpolatePair { first, second }),
            S::InterpolateVector(interpolate) => {
                Ok(R::InterpolateVector(interpolate.try_convert()?))
            }
            S::Lookup { lookup, default } => Ok(R::new_lookup(lookup, default)),
        }
    }
}

impl<T: IntInput, V: Interpolate> From<&Selector<T, V>> for SelectorSerde<T, V> {
    fn from(interpolator: &Selector<T, V>) -> Self {
        match interpolator {
            R::Const(value) => S::Const(value.clone()),
            R::InterpolatePair { first, second } => S::InterpolatePair {
                first: first.clone(),
                second: second.clone(),
            },
            R::InterpolateVector(interpolate) => S::InterpolateVector(interpolate.into()),
            R::Lookup { lookup, default } => S::Lookup {
                lookup: lookup.clone(),
                default: default.clone(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversion() {
        let start = Selector::Const(123);
        let serde: SelectorSerde<u32, u8> = (&start).into();

        assert_eq!(serde.try_convert().unwrap(), start)
    }
}

use crate::data::color::ColorSerde;
use crate::data::math::interpolation::vector::VectorInterpolatorSerde;
use anyhow::Result;
use omg::data::input::IntInput;
use omg::data::math::interpolation::Interpolate;
use omg::data::math::selector::Selector;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type ColorSelectorSerde = SelectorSerde<u8, ColorSerde>;

#[derive(Debug, Serialize, Deserialize)]
pub enum SelectorSerde<I: IntInput, V> {
    Const(V),
    InterpolatePair { first: V, second: V },
    InterpolateVector(VectorInterpolatorSerde<I, V>),
    Lookup { lookup: HashMap<I, V>, default: V },
}

type S<I, V> = SelectorSerde<I, V>;
type R<I, U> = Selector<I, U>;

impl<T: IntInput, V> SelectorSerde<T, V> {
    pub fn try_convert<U: Interpolate + From<V>>(self) -> Result<Selector<T, U>> {
        match self {
            S::Const(value) => Ok(R::Const(value.into())),
            S::InterpolatePair { first, second } => {
                Ok(R::new_interpolate_pair(first.into(), second.into()))
            }
            S::InterpolateVector(interpolate) => {
                Ok(R::InterpolateVector(interpolate.try_convert()?))
            }
            S::Lookup { lookup, default } => {
                let hash_map = lookup
                    .into_iter()
                    .map(|(input, value)| (input, value.into()))
                    .collect();
                Ok(R::new_lookup(hash_map, default.into()))
            }
        }
    }
}

impl<T: IntInput, V: From<U>, U: Interpolate> From<&Selector<T, U>> for SelectorSerde<T, V> {
    fn from(interpolator: &Selector<T, U>) -> Self {
        match interpolator {
            R::Const(value) => S::Const(value.clone().into()),
            R::InterpolatePair { first, second } => S::InterpolatePair {
                first: first.clone().into(),
                second: second.clone().into(),
            },
            R::InterpolateVector(interpolate) => S::InterpolateVector(interpolate.into()),
            R::Lookup { lookup, default } => {
                let lookup = lookup
                    .iter()
                    .map(|(input, value)| (*input, value.clone().into()))
                    .collect();
                S::Lookup {
                    lookup,
                    default: default.clone().into(),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use omg::data::color::{BLUE, RED};

    #[test]
    fn test_conversion() {
        let start = Selector::Const(123);
        let serde: SelectorSerde<u32, u8> = (&start).into();

        assert_eq!(serde.try_convert().unwrap(), start)
    }

    #[test]
    fn test_convert_color_selector() {
        let vector = vec![(0u8, RED), (200u8, BLUE)];
        let start = Selector::new_interpolate_vector(vector).unwrap();
        let serde: ColorSelectorSerde = (&start).into();

        assert_eq!(serde.try_convert().unwrap(), start)
    }
}

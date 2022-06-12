use crate::data::math::interpolation::vector::EntrySerde;
use anyhow::Result;
use omg::data::color::Color;
use omg::data::math::selector::ColorSelector;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub enum ColorSelectorSerde {
    Const(String),
    InterpolatePair {
        first: String,
        second: String,
    },
    InterpolateVector(Vec<EntrySerde<u8, String>>),
    Lookup {
        lookup: HashMap<u8, String>,
        default: String,
    },
}

type S = ColorSelectorSerde;
type R = ColorSelector;

impl ColorSelectorSerde {
    pub fn try_convert(self) -> Result<ColorSelector> {
        match self {
            S::Const(value) => {
                let color = Color::try_from(value)?;
                Ok(R::Const(color))
            }
            S::InterpolatePair { first, second } => {
                let first = Color::try_from(first)?;
                let second = Color::try_from(second)?;
                Ok(R::new_interpolate_pair(first, second))
            }
            S::InterpolateVector(interpolate) => {
                let vector: Result<Vec<(u8, Color)>> = interpolate
                    .into_iter()
                    .map(|e| e.value().clone().try_into().map(|c| (e.threshold(), c)))
                    .collect();
                Ok(R::new_interpolate_vector(vector?)?)
            }
            S::Lookup { lookup, default } => {
                let result: Result<Vec<(u8, Color)>> = lookup
                    .into_iter()
                    .map(|(input, value)| value.try_into().map(|c| (input, c)))
                    .collect();
                Ok(R::new_lookup(
                    result?.into_iter().collect(),
                    default.try_into()?,
                ))
            }
        }
    }
}

impl From<&ColorSelector> for ColorSelectorSerde {
    fn from(interpolator: &ColorSelector) -> Self {
        match interpolator {
            R::Const(value) => S::Const((*value).into()),
            R::InterpolatePair { first, second } => S::InterpolatePair {
                first: (*first).into(),
                second: (*second).into(),
            },
            R::InterpolateVector(interpolate) => {
                let vector: Vec<EntrySerde<u8, String>> = interpolate
                    .get_all()
                    .iter()
                    .map(|e| EntrySerde::new(e.threshold(), String::from(e.value())))
                    .collect();
                S::InterpolateVector(vector)
            }
            R::Lookup { lookup, default } => {
                let lookup = lookup
                    .iter()
                    .map(|(input, value)| (*input, (*value).into()))
                    .collect();
                S::Lookup {
                    lookup,
                    default: (*default).into(),
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
        let start = ColorSelector::Const(RED);
        let serde: ColorSelectorSerde = (&start).into();

        assert_eq!(serde.try_convert().unwrap(), start)
    }

    #[test]
    fn test_convert_color_selector() {
        let vector = vec![(0u8, RED), (200u8, BLUE)];
        let start = ColorSelector::new_interpolate_vector(vector).unwrap();
        let serde: ColorSelectorSerde = (&start).into();

        assert_eq!(serde.try_convert().unwrap(), start)
    }
}

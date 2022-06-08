use crate::data::input::IntInput;
use crate::data::math::interpolation::vector::VectorInterpolator;
use crate::data::math::interpolation::Interpolate;
use anyhow::Result;
use std::collections::HashMap;

/// Selects a value based on the input.
#[derive(Debug)]
pub enum Selector<I: IntInput, V: Interpolate> {
    /// Returns a specific element.
    ///
    /// ```
    ///# use omg::data::math::selector::Selector;
    /// assert_eq!(Selector::Const(99).get(128), 99);
    /// ```
    Const(V),
    /// Interpolates 2 elements.
    ///
    /// ```
    ///# use omg::data::math::selector::Selector;
    /// let selector = Selector::new_interpolate_pair(100, 200);
    ///
    /// assert_eq!(selector.get(128), 150);
    /// ```
    InterpolatePair { first: V, second: V },
    /// Interpolates multiple elements.
    ///
    /// ```
    ///# use omg::data::math::selector::Selector;
    /// let interpolator = Selector::new_interpolate_vector(vec![(100,150), (150,200), (200, 100)]).unwrap();
    ///
    /// assert_eq!(interpolator.get(125), 175);
    /// ```
    InterpolateVector(VectorInterpolator<I, V>),
    /// Looks the input up in a hashmap or returns the default value.
    ///
    /// ```
    ///# use omg::data::math::selector::Selector;
    /// let lookup = vec![(1u8, 25u8), (3, 100)].into_iter().collect();
    /// let selector = Selector::new_lookup(lookup, 1);
    ///
    /// assert_eq!(selector.get(0), 1);
    /// assert_eq!(selector.get(1), 25);
    /// assert_eq!(selector.get(2), 1);
    /// assert_eq!(selector.get(3), 100);
    /// assert_eq!(selector.get(4), 1);
    /// ```
    Lookup { lookup: HashMap<I, V>, default: V },
}

impl<I: IntInput, T: Interpolate> Selector<I, T> {
    pub fn new_interpolate_pair(first: T, second: T) -> Selector<I, T> {
        Selector::InterpolatePair { first, second }
    }

    pub fn new_interpolate_vector(vector: Vec<(I, T)>) -> Result<Selector<I, T>> {
        let interpolation = VectorInterpolator::new(vector)?;

        Ok(Selector::InterpolateVector(interpolation))
    }

    pub fn new_lookup(lookup: HashMap<I, T>, default: T) -> Selector<I, T> {
        Selector::Lookup { lookup, default }
    }

    /// Selects a value based on the input.
    pub fn get(&self, input: I) -> T {
        match self {
            Selector::Const(value) => value.clone(),
            Selector::InterpolatePair { first, second } => first.lerp(second, input.as_() / 255.0),
            Selector::InterpolateVector(interpolation) => interpolation.interpolate(input),
            Selector::Lookup { lookup, default } => lookup
                .get(&input)
                .cloned()
                .unwrap_or_else(|| default.clone()),
        }
    }
}

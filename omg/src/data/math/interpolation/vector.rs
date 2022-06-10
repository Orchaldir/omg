use crate::data::input::Input;
use crate::data::math::interpolation::Interpolate;
use anyhow::{bail, Result};

#[svgbobdoc::transform]
/// Interpolates multiple values based on their thresholds.
///
/// # Diagram
///
/// ```svgbob
///      value
///        ^
///        |                   *
///        |                  / \
///        |        *        /   \
///        |       / \      /     *--
///        |      /   *----*
///        |     /
///        |----*
///        |
///        +----*-----------------*--> threshold
///           first             last
/// ```
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct VectorInterpolator<T: Input, V: Interpolate> {
    vector: Vec<InterpolationEntry<T, V>>,
}

impl<T: Input, V: Interpolate> VectorInterpolator<T, V> {
    /// Returns an interpolator, if the vector is valid. It needs 2 or more values:
    ///
    /// ```
    ///# use omg::data::math::interpolation::vector::VectorInterpolator;
    /// assert!(VectorInterpolator::new(vec![(0u32,50)]).is_err());
    /// ```
    ///
    /// The values must be ordered based in their threshold:
    ///
    /// ```
    ///# use omg::data::math::interpolation::vector::VectorInterpolator;
    /// assert!(VectorInterpolator::new(vec![(50u32,50),(0,200)]).is_err());
    /// ```
    pub fn new(vector: Vec<(T, V)>) -> Result<VectorInterpolator<T, V>> {
        if vector.len() < 2 {
            bail!("The vector needs at least 2 elements!");
        }

        let mut last_threshold = T::zero();

        for (threshold, _) in &vector {
            if *threshold < last_threshold {
                bail!("The vector is not sorted!");
            }
            last_threshold = *threshold;
        }

        Ok(VectorInterpolator {
            vector: vector
                .into_iter()
                .map(|e| InterpolationEntry {
                    threshold: e.0,
                    value: e.1,
                })
                .collect::<Vec<_>>(),
        })
    }

    pub fn get_all(&self) -> &[InterpolationEntry<T, V>] {
        &self.vector
    }

    /// Interpolates the values based on the input.
    ///
    /// ```
    ///# use omg::data::math::interpolation::vector::VectorInterpolator;
    /// let interpolator = VectorInterpolator::new(vec![(100u32,150), (150,200)]).unwrap();
    ///
    /// assert_eq!(interpolator.interpolate(100), 150);
    /// assert_eq!(interpolator.interpolate(125), 175);
    /// assert_eq!(interpolator.interpolate(150), 200);
    /// ```
    pub fn interpolate(&self, input: T) -> V {
        let mut last_entry = self.vector.get(0).unwrap();

        if input <= last_entry.threshold {
            return last_entry.value.clone();
        }

        for entry in &self.vector[1..] {
            if input <= entry.threshold {
                return InterpolationEntry::interpolate(last_entry, entry, input);
            }

            last_entry = entry;
        }

        last_entry.value.clone()
    }
}

/// Stores the values & thresholds for [`VectorInterpolator`].
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct InterpolationEntry<T: Input, V: Interpolate> {
    threshold: T,
    value: V,
}

impl<T: Input, V: Interpolate> InterpolationEntry<T, V> {
    pub fn threshold(&self) -> T {
        self.threshold
    }

    pub fn value(&self) -> V {
        self.value.clone()
    }

    /// Interpolates between the values of 2 consecutive [`InterpolationEntry`] based on the input.
    pub fn interpolate(entry0: &Self, entry1: &Self, input: T) -> V {
        let factor_in_interval =
            (input - entry0.threshold).as_() / (entry1.threshold - entry0.threshold).as_();
        entry0.value.lerp(&entry1.value, factor_in_interval)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_before_the_first_element() {
        let interpolator = VectorInterpolator::new(vec![(100u32, 150), (150, 200)]).unwrap();

        assert_eq!(interpolator.interpolate(0), 150);
        assert_eq!(interpolator.interpolate(50), 150);
    }

    #[test]
    fn test_after_the_last_element() {
        let interpolator = VectorInterpolator::new(vec![(100u32, 150), (150, 200)]).unwrap();

        assert_eq!(interpolator.interpolate(200), 200);
        assert_eq!(interpolator.interpolate(250), 200);
    }

    #[test]
    fn test_with_float() {
        let interpolator =
            VectorInterpolator::new(vec![(100.0f32, 150), (150.0, 200), (200.0, 100)]).unwrap();

        assert_eq!(interpolator.interpolate(0.0), 150);
        assert_eq!(interpolator.interpolate(50.0), 150);
        assert_eq!(interpolator.interpolate(100.0), 150);
        assert_eq!(interpolator.interpolate(125.0), 175);
        assert_eq!(interpolator.interpolate(150.0), 200);
        assert_eq!(interpolator.interpolate(175.0), 150);
        assert_eq!(interpolator.interpolate(200.0), 100);
        assert_eq!(interpolator.interpolate(255.0), 100);
    }
}

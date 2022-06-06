use crate::data::math::interpolation::Interpolate;
use num_traits::int::PrimInt;
use num_traits::AsPrimitive;

pub trait Threshold: PrimInt + AsPrimitive<f32> + Clone + Copy {}

impl Threshold for u8 {}
impl Threshold for u32 {}

/// A helper struct for [`VectorInterpolator`].
#[derive(Debug, PartialEq, Eq, Clone)]
struct InterpolationEntry<T: Threshold, V: Interpolate> {
    threshold: T,
    value: V,
}

impl<T: Threshold, V: Interpolate> InterpolationEntry<T, V> {
    /// Interpolates between the values of 2 consecutive [`InterpolationEntry`] based on the input.
    pub fn interpolate(entry0: &Self, entry1: &Self, input: T) -> V {
        let factor_in_interval =
            (input - entry0.threshold).as_() / (entry1.threshold - entry0.threshold).as_();
        entry0.value.lerp(&entry1.value, factor_in_interval)
    }
}

#[svgbobdoc::transform]
/// Interpolates multiple values.
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
pub struct VectorInterpolator<T: Threshold, V: Interpolate> {
    vector: Vec<InterpolationEntry<T, V>>,
}

impl<T: Threshold, V: Interpolate> VectorInterpolator<T, V> {
    /// Returns a interpolator, if the input is valid. It needs 2 or more values:
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
    pub fn new(vector: Vec<(T, V)>) -> Result<VectorInterpolator<T, V>, &'static str> {
        if vector.len() < 2 {
            return Err("The vector needs at least 2 elements!");
        }

        let mut last_value = T::zero();

        for (value, _) in &vector {
            if *value < last_value {
                return Err("The elements of vector are not ordered!");
            }
            last_value = *value;
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

    /// Interpolates the values based on the input.
    ///
    /// ```
    ///# use omg::data::math::interpolation::vector::VectorInterpolator;
    /// let interpolator = VectorInterpolator::new(vec![(100u32,150), (150,200), (200, 100)]).unwrap();
    ///
    /// assert_eq!(interpolator.interpolate(  0), 150);
    /// assert_eq!(interpolator.interpolate( 50), 150);
    /// assert_eq!(interpolator.interpolate(100), 150);
    /// assert_eq!(interpolator.interpolate(125), 175);
    /// assert_eq!(interpolator.interpolate(150), 200);
    /// assert_eq!(interpolator.interpolate(175), 150);
    /// assert_eq!(interpolator.interpolate(200), 100);
    /// assert_eq!(interpolator.interpolate(255), 100);
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

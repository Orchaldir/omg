use crate::data::input::IntInput;

/// Overwrites the input if above or below a threshold.
#[derive(new, Debug, PartialEq, Eq, Clone, Copy)]
pub struct OverwriteWithThreshold<T: IntInput> {
    value: T,
    threshold: T,
}

impl<T: IntInput> OverwriteWithThreshold<T> {
    pub fn value(&self) -> T {
        self.value
    }

    pub fn threshold(&self) -> T {
        self.threshold
    }

    /// Overwrites the input if equal or above a threshold.
    ///
    /// ```
    ///# use omg::data::math::transformer::threshold::OverwriteWithThreshold;
    /// let overwrite = OverwriteWithThreshold::new(42, 100);
    ///
    /// assert_eq!(overwrite.overwrite_if_above(  0),  0);
    /// assert_eq!(overwrite.overwrite_if_above( 99), 99);
    /// assert_eq!(overwrite.overwrite_if_above(100), 42);
    /// assert_eq!(overwrite.overwrite_if_above(101), 42);
    /// assert_eq!(overwrite.overwrite_if_above(255), 42);
    /// ```
    pub fn overwrite_if_above(&self, input: T) -> T {
        if input >= self.threshold {
            self.value
        } else {
            input
        }
    }

    /// Overwrites the output, if the input is equal or above a threshold.
    ///
    /// ```
    ///# use omg::data::math::transformer::threshold::OverwriteWithThreshold;
    /// let overwrite = OverwriteWithThreshold::new(42, 100);
    ///
    /// assert_eq!(overwrite.overwrite_output_if_above(  0, 200), 200);
    /// assert_eq!(overwrite.overwrite_output_if_above( 99, 199), 199);
    /// assert_eq!(overwrite.overwrite_output_if_above(100, 198),  42);
    /// assert_eq!(overwrite.overwrite_output_if_above(101, 197),  42);
    /// assert_eq!(overwrite.overwrite_output_if_above(255, 196),  42);
    /// ```
    pub fn overwrite_output_if_above(&self, input: T, output: T) -> T {
        if input >= self.threshold {
            self.value
        } else {
            output
        }
    }

    /// Overwrites the input if equal or above a threshold.
    ///
    /// ```
    ///# use omg::data::math::transformer::threshold::OverwriteWithThreshold;
    /// let overwrite = OverwriteWithThreshold::new(42, 100);
    ///
    /// assert_eq!(overwrite.overwrite_if_below(  0),  42);
    /// assert_eq!(overwrite.overwrite_if_below( 99),  42);
    /// assert_eq!(overwrite.overwrite_if_below(100),  42);
    /// assert_eq!(overwrite.overwrite_if_below(101), 101);
    /// assert_eq!(overwrite.overwrite_if_below(255), 255);
    /// ```
    pub fn overwrite_if_below(&self, input: T) -> T {
        if input <= self.threshold {
            self.value
        } else {
            input
        }
    }

    /// Overwrites the output, if the input is equal or above a threshold.
    ///
    /// ```
    ///# use omg::data::math::transformer::threshold::OverwriteWithThreshold;
    /// let overwrite = OverwriteWithThreshold::new(42, 100);
    ///
    /// assert_eq!(overwrite.overwrite_output_if_below(  0, 200),  42);
    /// assert_eq!(overwrite.overwrite_output_if_below( 99, 199),  42);
    /// assert_eq!(overwrite.overwrite_output_if_below(100, 198),  42);
    /// assert_eq!(overwrite.overwrite_output_if_below(101, 197), 197);
    /// assert_eq!(overwrite.overwrite_output_if_below(255, 196), 196);
    /// ```
    pub fn overwrite_output_if_below(&self, input: T, output: T) -> T {
        if input <= self.threshold {
            self.value
        } else {
            output
        }
    }
}

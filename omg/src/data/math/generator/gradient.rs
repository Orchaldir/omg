use crate::data::math::interpolation::lerp;
use anyhow::{bail, Result};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Gradient {
    start: u32,
    length: u32,
    value_start: u8,
    value_end: u8,
}

impl Gradient {
    /// Creates a gradient unless the length is 0:
    ///
    /// ```
    ///# use omg::data::math::generator::gradient::Gradient;
    /// assert!(Gradient::new(1000, 0, 200, 100).is_err());
    /// ```
    ///
    /// The values must be ordered based in their threshold:
    ///
    /// ```
    ///# use omg::data::math::generator::gradient::Gradient;
    /// assert!(Gradient::new(1000, 300, 200, 200).is_err());
    /// ```
    pub fn new(start: u32, length: u32, value_start: u8, value_end: u8) -> Result<Gradient> {
        if length == 0 {
            bail!("The length of the gradient is 0!");
        } else if value_start == value_end {
            bail!("The start & end value of the gradient are equal!");
        }

        Ok(Gradient {
            start,
            length,
            value_start,
            value_end,
        })
    }

    pub fn start(&self) -> u32 {
        self.start
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn value_start(&self) -> u8 {
        self.value_start
    }

    pub fn value_end(&self) -> u8 {
        self.value_end
    }

    /// Generates the gradient.
    pub fn generate(&self, input: u32) -> u8 {
        if input <= self.start {
            return self.value_start;
        }
        let distance = (input - self.start) as f32;
        let factor = distance / self.length as f32;
        lerp(self.value_start, self.value_end, factor)
    }

    /// Generates the absolute gradient.
    pub fn generate_absolute(&self, input: u32) -> u8 {
        let distance = self.start.abs_diff(input) as f32;
        let factor = distance / self.length as f32;
        lerp(self.value_start, self.value_end, factor)
    }
}

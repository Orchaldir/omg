use crate::data::math::generator::gradient::Gradient;
use crate::data::math::interpolation::vector::VectorInterpolator;
use Generator1d::*;

#[svgbobdoc::transform]
/// Generates values for a 1d input.
pub enum Generator1d {
    /// Generates a linear gradient between a center and both sides.
    ///
    /// # Diagram
    ///
    /// ```svgbob
    ///      value
    ///        ^
    ///        |
    ///        |        center
    /// center |        *
    ///        |       / \
    ///        |      /   \
    ///        |     /     \
    ///    end |----*       *----
    ///        |
    ///        +----*-------*---> input
    ///         -length    +length
    /// ```
    ///
    /// # Example
    ///
    /// ```
    ///# use omg::data::math::generator::generator1d::Generator1d;
    ///# use omg::data::math::generator::gradient::Gradient;
    /// let gradient = Gradient::new(100, 0, 80, 100);
    /// let generator = Generator1d::AbsoluteGradient(gradient);
    ///
    /// assert_eq!(generator.generate(  0),  20);
    /// assert_eq!(generator.generate(  1),  21);
    /// assert_eq!(generator.generate( 79),  99);
    /// assert_eq!(generator.generate( 80), 100);
    /// assert_eq!(generator.generate( 81),  99);
    /// assert_eq!(generator.generate(130),  50);
    /// assert_eq!(generator.generate(179),   1);
    /// assert_eq!(generator.generate(180),   0);
    /// assert_eq!(generator.generate(181),   0);
    /// assert_eq!(generator.generate(u32::MAX), 0);
    /// ```
    AbsoluteGradient(Gradient),
    /// Generates a linear gradient between a start and an end value.
    ///
    /// # Diagram
    ///
    /// ```svgbob
    ///     value
    ///       ^
    ///       |
    ///       |
    ///   end |        *------
    ///       |       /
    ///       |      /
    ///       |     /
    /// start |----*
    ///       |
    ///       +----*---*------> input
    ///         start  end
    /// ```
    ///
    /// # Example
    ///
    ///```
    ///# use omg::data::math::generator::generator1d::Generator1d;
    ///# use omg::data::math::generator::gradient::Gradient;
    /// let gradient = Gradient::new(100, 200, 1000, 100);
    /// let generator = Generator1d::Gradient(gradient);
    ///
    /// assert_eq!(generator.generate(   0), 100);
    /// assert_eq!(generator.generate( 500), 100);
    /// assert_eq!(generator.generate(1000), 100);
    /// assert_eq!(generator.generate(1001), 101);
    /// assert_eq!(generator.generate(1050), 150);
    /// assert_eq!(generator.generate(1099), 199);
    /// assert_eq!(generator.generate(1100), 200);
    /// assert_eq!(generator.generate(1101), 200);
    /// assert_eq!(generator.generate(1200), 200);
    ///```
    Gradient(Gradient),
    /// Returns the input as output until it reaches the maximum.
    ///
    /// # Example
    ///
    ///```
    ///# use omg::data::math::generator::generator1d::Generator1d::InputAsOutput;
    ///
    /// assert_eq!(InputAsOutput.generate(0), 0);
    /// assert_eq!(InputAsOutput.generate(1), 1);
    /// assert_eq!(InputAsOutput.generate(2), 2);
    ///```
    InputAsOutput,
    /// Interpolates multiple elements.
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
    ///        +----*-----------------*--> input
    ///           first             last
    /// ```
    ///
    /// # Example
    ///
    /// ```
    ///# use omg::data::math::generator::generator1d::Generator1d::InterpolateVector;
    ///# use omg::data::math::interpolation::vector::VectorInterpolator;
    /// let interpolator = VectorInterpolator::new(vec![(100u32,150), (150,200), (200, 100)]). unwrap();
    /// let generator = InterpolateVector(interpolator);
    ///
    /// assert_eq!(generator.generate(125), 175);
    /// ```
    InterpolateVector(VectorInterpolator<u32, u8>),
}

impl Generator1d {
    /// Generates an output for an input.
    pub fn generate(&self, input: u32) -> u8 {
        match self {
            AbsoluteGradient(gradient) => gradient.generate_absolute(input),
            Gradient(gradient) => gradient.generate(input),
            InputAsOutput => {
                if input > u8::MAX as u32 {
                    return u8::MAX;
                }

                input as u8
            }
            InterpolateVector(interpolator) => interpolator.interpolate(input),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_bigger_than_max_output() {
        assert_eq!(InputAsOutput.generate(300), 255);
        assert_eq!(InputAsOutput.generate(u32::MAX), 255);
    }
}

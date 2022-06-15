use crate::data::math::distance::calculate_distance;
use crate::data::math::generator::generator1d::Generator1d;
use crate::data::math::generator::noise::Noise;
use crate::data::math::size2d::Size2d;
use Generator2d::*;

#[svgbobdoc::transform]
/// Generate values for a 2d input.
/// Used for the procedural generation of 2d maps.
#[derive(Debug, PartialEq, Clone)]
pub enum Generator2d {
    /// Feeds the x values to a [`Generator1d`].
    ///
    /// ```
    ///# use omg_core::data::math::generator::generator1d::Generator1d;
    ///# use omg_core::data::math::generator::generator2d::Generator2d;
    /// let generator = Generator2d::new_apply_to_x(Generator1d::InputAsOutput);
    ///
    /// assert_eq!(generator.generate(0, 0), 0);
    /// assert_eq!(generator.generate(1, 0), 1);
    /// assert_eq!(generator.generate(2, 0), 2);
    /// assert_eq!(generator.generate(0, 1), 0);
    /// assert_eq!(generator.generate(1, 1), 1);
    /// assert_eq!(generator.generate(2, 1), 2);
    /// assert_eq!(generator.generate(0, 2), 0);
    /// assert_eq!(generator.generate(1, 2), 1);
    /// assert_eq!(generator.generate(2, 2), 2);
    /// ```
    ApplyToX(Generator1d),
    /// Feeds the y values to a [`Generator1d`].
    ///
    /// ```
    ///# use omg_core::data::math::generator::generator1d::Generator1d;
    ///# use omg_core::data::math::generator::generator2d::Generator2d;
    /// let generator = Generator2d::new_apply_to_y(Generator1d::InputAsOutput);
    ///
    /// assert_eq!(generator.generate(0, 0), 0);
    /// assert_eq!(generator.generate(1, 0), 0);
    /// assert_eq!(generator.generate(2, 0), 0);
    /// assert_eq!(generator.generate(0, 1), 1);
    /// assert_eq!(generator.generate(1, 1), 1);
    /// assert_eq!(generator.generate(2, 1), 1);
    /// assert_eq!(generator.generate(0, 2), 2);
    /// assert_eq!(generator.generate(1, 2), 2);
    /// assert_eq!(generator.generate(2, 2), 2);
    /// ```
    ApplyToY(Generator1d),
    /// Feeds the distance from a point to a [`Generator1d`].
    ///
    /// ```
    ///# use omg_core::data::math::generator::generator1d::Generator1d;
    ///# use omg_core::data::math::generator::generator2d::Generator2d;
    /// let generator = Generator2d::new_apply_to_distance(Generator1d::InputAsOutput, 10, 5);
    ///
    /// assert_eq!(generator.generate(10,  5), 0);
    /// assert_eq!(generator.generate(10,  0), 5);
    /// assert_eq!(generator.generate(10, 10), 5);
    /// assert_eq!(generator.generate( 5,  5), 5);
    /// assert_eq!(generator.generate(15,  5), 5);
    /// ```
    ApplyToDistance {
        generator: Generator1d,
        center_x: u32,
        center_y: u32,
    },
    /// Generates the index of each 2d point.
    ///
    /// ```
    ///# use omg_core::data::math::generator::generator2d::Generator2d::IndexGenerator;
    ///# use omg_core::data::math::size2d::Size2d;
    /// let generator = IndexGenerator(Size2d::unchecked(2, 3));
    ///
    /// assert_eq!(generator.generate(0, 0), 0);
    /// assert_eq!(generator.generate(1, 0), 1);
    /// assert_eq!(generator.generate(0, 1), 2);
    /// assert_eq!(generator.generate(1, 1), 3);
    /// assert_eq!(generator.generate(0, 2), 4);
    /// assert_eq!(generator.generate(1, 2), 5);
    /// ```
    IndexGenerator(Size2d),
    /// Generates 2d noise.
    Noise2d(Noise),
}

impl Generator2d {
    pub fn new_apply_to_x(generator: Generator1d) -> Generator2d {
        ApplyToX(generator)
    }

    pub fn new_apply_to_y(generator: Generator1d) -> Generator2d {
        ApplyToY(generator)
    }

    pub fn new_apply_to_distance(generator: Generator1d, x: u32, y: u32) -> Generator2d {
        ApplyToDistance {
            generator,
            center_x: x,
            center_y: y,
        }
    }

    /// Generates a value for a 2d point (x,y).
    pub fn generate(&self, x: u32, y: u32) -> u8 {
        match self {
            ApplyToX(generator) => generator.generate(x),
            ApplyToY(generator) => generator.generate(y),
            ApplyToDistance {
                generator,
                center_x,
                center_y,
            } => {
                let distance = calculate_distance(*center_x, *center_y, x, y);
                generator.generate(distance)
            }
            IndexGenerator(size) => size.saturating_to_index(x, y) as u8,
            Noise2d(noise) => noise.generate2d(x, y),
        }
    }
}

use noise::{NoiseFn, Seedable, SuperSimplex};
use anyhow::{bail, Result};

/// Hide the noise functions from library [`noise`].
#[derive(Clone, Debug)]
pub struct Noise {
    algo: Box<SuperSimplex>,
    scale: f64,
    base: f64,
    factor: f64,
}

impl Noise {
    /// Tries to create a noise generator.
    ///
    /// # Arguments
    ///
    /// * `seed` - The number to initialize the pseudorandom number generator.
    /// * `scale` - How big are the generated features in pixel?
    /// * `min_value` - The minimum of the generated values.
    /// * `max_value` - The maximum of the generated values.
    ///
    /// # Examples
    ///
    /// Fails if scale is negative:
    ///
    ///```
    ///# use omg::data::math::generator::noise::Noise;
    /// assert!(Noise::new(0, -1.0, 0, 255).is_err())
    ///```
    /// Also fails if min_value >= max_value:
    ///
    ///```
    ///# use omg::data::math::generator::noise::Noise;
    /// assert!(Noise::new(0, 5.0, 200, 105).is_err())
    ///```
    ///
    pub fn new(seed: u32, scale: f64, min_value: u8, max_value: u8) -> Result<Noise> {
        if scale <= 0.0 {
            bail!("The noise's scale must not be negative!");
        } else if min_value >= max_value {
            bail!("The noise's minimum must be below its maximum");
        }

        Ok(Noise {
            algo: Box::new(SuperSimplex::new().set_seed(seed)),
            scale,
            base: 1.0 + min_value as f64 / 255.0,
            factor: (max_value - min_value) as f64 / 2.0,
        })
    }

    /// Generates noise for an input.
    pub fn generate1d(&self, input: u32) -> u8 {
        let input = input as f64 / self.scale;
        let positive_value = self.algo.get([input, 0.0]) + self.base;
        (positive_value * self.factor) as u8
    }

    /// Generates noise for a 2d point.
    pub fn generate2d(&self, x: u32, y: u32) -> u8 {
        let x = x as f64 / self.scale;
        let y = y as f64 / self.scale;
        let positive_value = self.algo.get([x, y]) + self.base;
        (positive_value * self.factor) as u8
    }
}

impl PartialEq for Noise {
    fn eq(&self, other: &Self) -> bool {
        self.base == other.base && self.factor == other.factor && self.scale == other.scale
    }
}

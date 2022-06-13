use anyhow::{bail, Result};
use noise::{NoiseFn, Seedable, SuperSimplex};

/// Hide the noise functions from library [`noise`].
#[derive(Clone, Debug)]
pub struct Noise {
    algo: Box<SuperSimplex>,
    scale: u32,
    scale_f64: f64,
    min_value: u8,
    max_value: u8,
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
    /// assert!(Noise::new(0, 0, 0, 255).is_err())
    ///```
    /// Also fails if min_value >= max_value:
    ///
    ///```
    ///# use omg::data::math::generator::noise::Noise;
    /// assert!(Noise::new(0, 5, 200, 105).is_err())
    ///```
    ///
    pub fn new(seed: u32, scale: u32, min_value: u8, max_value: u8) -> Result<Noise> {
        if scale == 0 {
            bail!("The noise's scale must be greater 0!");
        } else if min_value >= max_value {
            bail!("The noise's minimum must be below its maximum");
        }

        Ok(Noise {
            algo: Box::new(SuperSimplex::new().set_seed(seed)),
            scale,
            scale_f64: scale as f64,
            min_value,
            max_value,
            base: 1.0 + min_value as f64 / 255.0,
            factor: (max_value - min_value) as f64 / 2.0,
        })
    }

    pub fn seed(&self) -> u32 {
        self.algo.seed()
    }

    pub fn scale(&self) -> u32 {
        self.scale
    }

    pub fn min_value(&self) -> u8 {
        self.min_value
    }

    pub fn max_value(&self) -> u8 {
        self.max_value
    }

    /// Generates noise for an input.
    pub fn generate1d(&self, input: u32) -> u8 {
        let input = input as f64 / self.scale_f64;
        let positive_value = self.algo.get([input, 0.0]) + self.base;
        (positive_value * self.factor) as u8
    }

    /// Generates noise for a 2d point.
    pub fn generate2d(&self, x: u32, y: u32) -> u8 {
        let x = x as f64 / self.scale_f64;
        let y = y as f64 / self.scale_f64;
        let positive_value = self.algo.get([x, y]) + self.base;
        (positive_value * self.factor) as u8
    }
}

impl PartialEq for Noise {
    fn eq(&self, other: &Self) -> bool {
        self.seed() == other.seed()
            && self.scale == other.scale
            && self.min_value == other.min_value
            && self.max_value == other.max_value
    }
}

use std::ops::Sub;

use crate::data::map::Map2d;
use crate::data::math::size2d::Size2d;
use crate::data::name::validate_name;
use crate::generation::step::GenerationStep;
use anyhow::{bail, Result};

pub mod attributes;
pub mod step;

/// Generates a map based on a number of steps.
#[derive(Debug, PartialEq, Clone)]
pub struct MapGeneration {
    name: String,
    size: Size2d,
    steps: Vec<GenerationStep>,
}

impl MapGeneration {
    /// Creates a map generation, unless the name is invalid:
    ///
    /// ```
    ///# use omg::generation::MapGeneration;
    ///# use omg::data::math::size2d::Size2d;
    ///# use omg::generation::step::GenerationStep;
    /// let size = Size2d::unchecked(1, 2);
    /// let steps = vec![GenerationStep::debug("a"), GenerationStep::debug("b")];
    ///
    /// assert!(MapGeneration::new("", size, steps).is_err());
    /// ```
    ///
    /// Also invalid, if it has less than 2 steps:
    ///
    /// ```
    ///# use omg::generation::MapGeneration;
    ///# use omg::data::math::size2d::Size2d;
    ///# use omg::generation::step::GenerationStep;
    /// let size = Size2d::unchecked(1, 2);
    /// let steps = vec![GenerationStep::debug("a")];
    ///
    /// assert!(MapGeneration::new("map0", size, steps).is_err());
    /// ```
    pub fn new<S: Into<String>>(
        name: S,
        size: Size2d,
        steps: Vec<GenerationStep>,
    ) -> Result<MapGeneration> {
        let name = validate_name(name)?;

        if steps.len() < 2 {
            bail!("Map generator '{}' has too few steps!", name);
        }

        Ok(MapGeneration { name, size, steps })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn size(&self) -> &Size2d {
        &self.size
    }

    pub fn steps(&self) -> &[GenerationStep] {
        &self.steps
    }

    /// Generates the map.
    pub fn generate(&self) -> Map2d {
        let start = std::time::Instant::now();

        info!(
            "Generate the map '{}' with {:?} in {} steps:",
            self.name,
            self.size,
            self.steps.len(),
        );

        let mut start_step = start;
        let mut map = Map2d::with_name(self.name.clone(), self.size);

        self.steps.iter().for_each(|step| {
            step.run(&mut map);
            let end_step = std::time::Instant::now();
            let duration = end_step.sub(start_step);
            debug!("Step took {:?}", duration);
            start_step = end_step;
        });

        let end = std::time::Instant::now();
        let duration = end.sub(start);

        info!("Finished generation of '{}' in {:?}", self.name, duration);

        map
    }
}

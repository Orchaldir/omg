use crate::data::math::size2d::Size2dSerde;
use crate::generation::step::{FromStep, GenerationStepSerde, ToStep};
use anyhow::{Context, Result};
use omg::generation::MapGeneration;
use serde::{Deserialize, Serialize};

pub mod attributes;
pub mod step;

#[derive(Debug, Serialize, Deserialize)]
pub struct MapGenerationSerde {
    name: String,
    size: Size2dSerde,
    steps: Vec<GenerationStepSerde>,
}

impl MapGenerationSerde {
    pub fn try_convert(self) -> Result<MapGeneration> {
        let mut attributes: Vec<String> = Vec::new();
        let steps: Result<Vec<_>> = self
            .steps
            .into_iter()
            .enumerate()
            .map(|(index, step)| {
                step.try_convert(&mut attributes)
                    .with_context(|| format!("Failed to convert the {}.step!", index))
            })
            .collect();
        let steps = steps?;
        let size = self.size.try_convert()?;

        Ok(MapGeneration::new(self.name, size, steps))
    }
}

impl From<&MapGeneration> for MapGenerationSerde {
    fn from(map_generation: &MapGeneration) -> Self {
        let attributes: Vec<String> = Vec::new();
        let steps: Vec<GenerationStepSerde> = map_generation
            .steps()
            .iter()
            .map(|data| data.convert(&attributes))
            .collect();
        MapGenerationSerde {
            name: map_generation.name().to_string(),
            size: map_generation.size().into(),
            steps,
        }
    }
}

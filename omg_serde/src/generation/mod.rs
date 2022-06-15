use crate::data::math::size2d::Size2dSerde;
use crate::generation::step::{FromStep, GenerationStepSerde, ToStep};
use anyhow::{Context, Result};
use omg_core::generation::MapGenerator;
use serde::{Deserialize, Serialize};

pub mod attributes;
pub mod step;

#[derive(new, Debug, Serialize, Deserialize)]
pub struct MapGenerationSerde {
    name: String,
    size: Size2dSerde,
    steps: Vec<GenerationStepSerde>,
}

impl MapGenerationSerde {
    pub fn try_convert(self) -> Result<MapGenerator> {
        let mut attributes: Vec<String> = Vec::new();
        let steps: Result<Vec<_>> = self
            .steps
            .into_iter()
            .enumerate()
            .map(|(index, step)| {
                step.try_convert(&mut attributes)
                    .with_context(|| format!("Failed to convert the {}.step!", index + 1))
            })
            .collect();
        let steps = steps?;
        let size = self.size.try_convert()?;

        MapGenerator::new(self.name, size, steps)
    }
}

impl From<&MapGenerator> for MapGenerationSerde {
    fn from(map_generation: &MapGenerator) -> Self {
        let mut attributes: Vec<String> = Vec::new();
        let steps: Vec<GenerationStepSerde> = map_generation
            .steps()
            .iter()
            .map(|data| data.convert(&mut attributes))
            .collect();
        MapGenerationSerde {
            name: map_generation.name().to_string(),
            size: map_generation.size().into(),
            steps,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generation::attributes::modify::ModifyWithAttributeStepSerde;
    use omg_core::data::math::size2d::Size2d;
    use omg_core::generation::attributes::create::CreateAttributeStep;
    use omg_core::generation::attributes::modify::ModifyWithAttributeStep;
    use omg_core::generation::step::GenerationStep;

    #[test]
    fn test_conversion() {
        let create0 =
            GenerationStep::CreateAttribute(CreateAttributeStep::new("source", 0).unwrap());
        let create1 =
            GenerationStep::CreateAttribute(CreateAttributeStep::new("target", 0).unwrap());
        let modify = ModifyWithAttributeStep::new(0, 1, 100, 10);
        let modify = GenerationStep::ModifyWithAttribute(modify);
        let steps = vec![create0, create1, modify];
        let generation = MapGenerator::new("map", Size2d::unchecked(4, 5), steps).unwrap();

        let serde: MapGenerationSerde = (&generation).into();

        assert_eq!(serde.try_convert().unwrap(), generation);
    }

    #[test]
    fn test_conversion_missing_attribute() {
        let modify =
            ModifyWithAttributeStepSerde::new("source".to_string(), "target".to_string(), 100, 10);
        let modify = GenerationStepSerde::ModifyWithAttribute(modify);
        let size = Size2dSerde::new(4, 5);
        let serde = MapGenerationSerde::new("map".to_string(), size, vec![modify]);
        let result: Result<MapGenerator> = serde.try_convert();

        assert!(result.is_err());
    }
}

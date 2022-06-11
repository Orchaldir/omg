use crate::data::math::generator::generator2d::Generator2dSerde;
use crate::generation::step::{get_attribute_id, FromStep, ToStep};
use anyhow::{Context, Result};
use omg::generation::attributes::generator::GeneratorStep;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GeneratorStepSerde {
    name: String,
    attribute: String,
    generator: Generator2dSerde,
}

impl GeneratorStepSerde {
    fn inner_convert(self, attributes: &[String]) -> Result<GeneratorStep> {
        let id = get_attribute_id(&self.attribute, attributes)?;
        let generator = self.generator.try_convert()?;
        GeneratorStep::new(self.name, id, generator)
    }
}

impl ToStep<GeneratorStep> for GeneratorStepSerde {
    fn try_convert(self, attributes: &[String]) -> Result<GeneratorStep> {
        self.inner_convert(attributes)
            .context("Failed to convert to GeneratorStep!")
    }
}

impl FromStep<GeneratorStepSerde> for GeneratorStep {
    fn convert(&self, attributes: &[String]) -> GeneratorStepSerde {
        let attribute = attributes[self.attribute_id()].clone();
        GeneratorStepSerde {
            name: self.name().to_string(),
            attribute,
            generator: self.generator().into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generation::step::assert_eq;
    use omg::data::math::generator::generator2d::Generator2d;
    use omg::data::math::size2d::Size2d;

    #[test]
    fn test_conversion() {
        let attributes = vec!["test".to_string()];
        let generator = Generator2d::IndexGenerator(Size2d::unchecked(1, 2));
        let step = GeneratorStep::new("step", 0, generator).unwrap();

        assert_eq(step, &attributes);
    }
}

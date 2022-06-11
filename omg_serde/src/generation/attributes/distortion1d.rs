use crate::data::math::generator::generator1d::Generator1dSerde;
use crate::generation::step::{get_attribute_id, FromStep, ToStep};
use anyhow::{Context, Result};
use omg::generation::attributes::distortion1d::Distortion1dStep;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Distortion1dStepSerde {
    attribute: String,
    generator: Generator1dSerde,
}

impl Distortion1dStepSerde {
    fn inner_convert(self, attributes: &[String]) -> Result<Distortion1dStep> {
        let id = get_attribute_id(&self.attribute, attributes)?;
        let generator = self.generator.try_convert()?;
        Ok(Distortion1dStep::new(id, generator))
    }
}

impl ToStep<Distortion1dStep> for Distortion1dStepSerde {
    fn try_convert(self, attributes: &mut Vec<String>) -> Result<Distortion1dStep> {
        self.inner_convert(attributes)
            .context("Failed to convert to Distortion1dStep!")
    }
}

impl FromStep<Distortion1dStepSerde> for Distortion1dStep {
    fn convert(&self, attributes: &[String]) -> Distortion1dStepSerde {
        let attribute = attributes[self.attribute_id()].clone();
        Distortion1dStepSerde {
            attribute,
            generator: self.generator().into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generation::step::assert_eq;
    use omg::data::math::generator::generator1d::Generator1d;

    #[test]
    fn test_conversion() {
        let mut attributes = vec!["test".to_string()];
        let step = Distortion1dStep::new(0, Generator1d::InputAsOutput);

        assert_eq(step, &mut attributes);
    }
}

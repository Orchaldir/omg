use crate::data::math::generator::generator2d::Generator2dSerde;
use crate::generation::step::{get_attribute_id, FromStep, ToStep};
use anyhow::Result;
use omg::generation::attributes::distortion2d::Distortion2dStep;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Distortion2dStepSerde {
    attribute: String,
    generator_x: Generator2dSerde,
    generator_y: Generator2dSerde,
}

impl ToStep<Distortion2dStep> for Distortion2dStepSerde {
    fn try_convert(self, attributes: &[String]) -> Result<Distortion2dStep> {
        let id = get_attribute_id(&self.attribute, attributes)?;
        let generator_x = self.generator_x.try_convert()?;
        let generator_y = self.generator_y.try_convert()?;
        Ok(Distortion2dStep::new(id, generator_x, generator_y))
    }
}

impl FromStep<Distortion2dStepSerde> for Distortion2dStep {
    fn convert(&self, attributes: &[String]) -> Distortion2dStepSerde {
        let attribute = attributes[self.attribute_id()].clone();
        Distortion2dStepSerde {
            attribute,
            generator_x: self.generator_x().into(),
            generator_y: self.generator_y().into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generation::step::assert_eq;
    use omg::data::math::generator::generator2d::Generator2d::IndexGenerator;
    use omg::data::math::size2d::Size2d;

    #[test]
    fn test_conversion() {
        let attributes = vec!["test".to_string()];
        let generator_x = IndexGenerator(Size2d::unchecked(1, 2));
        let generator_y = IndexGenerator(Size2d::unchecked(3, 4));
        let step = Distortion2dStep::new(0, generator_x, generator_y);

        assert_eq(step, &attributes);
    }
}

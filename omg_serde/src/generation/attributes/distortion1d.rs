use crate::data::math::generator::generator1d::Generator1dSerde;
use crate::generation::step::{get_attribute_id, FromStep, ToStep};
use anyhow::Result;
use omg::generation::attributes::distortion1d::Distortion1d;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Distortion1dSerde {
    attribute: String,
    generator: Generator1dSerde,
}

impl ToStep<Distortion1d> for Distortion1dSerde {
    fn try_convert(self, attributes: &[String]) -> Result<Distortion1d> {
        let id = get_attribute_id(&self.attribute, attributes)?;
        let generator = self.generator.try_convert()?;
        Ok(Distortion1d::new(id, generator))
    }
}

impl FromStep<Distortion1dSerde> for Distortion1d {
    fn convert(&self, attributes: &[String]) -> Distortion1dSerde {
        let attribute = attributes[self.attribute_id()].clone();
        Distortion1dSerde {
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
        let attributes = vec!["test".to_string()];
        let step = Distortion1d::new(0, Generator1d::InputAsOutput);

        assert_eq(step, &attributes);
    }
}

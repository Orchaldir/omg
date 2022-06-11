use crate::data::math::transformer::transformer2d::Transformer2dSerde;
use crate::generation::step::{get_attribute_id, FromStep, ToStep};
use anyhow::Result;
use omg::generation::attributes::transformer::TransformAttribute2dStep;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TransformAttribute2dStepSerde {
    name: String,
    source0: String,
    source1: String,
    target: String,
    transformer: Transformer2dSerde,
}

impl ToStep<TransformAttribute2dStep> for TransformAttribute2dStepSerde {
    fn try_convert(self, attributes: &[String]) -> Result<TransformAttribute2dStep> {
        let source_id0 = get_attribute_id(&self.source0, attributes)?;
        let source_id1 = get_attribute_id(&self.source1, attributes)?;
        let target_id = get_attribute_id(&self.target, attributes)?;
        let transformer = self.transformer.try_convert()?;

        TransformAttribute2dStep::new(self.name, source_id0, source_id1, target_id, transformer)
    }
}

impl FromStep<TransformAttribute2dStepSerde> for TransformAttribute2dStep {
    fn convert(&self, attributes: &[String]) -> TransformAttribute2dStepSerde {
        let source0 = attributes[self.source_id0()].clone();
        let source1 = attributes[self.source_id1()].clone();
        let target = attributes[self.target_id()].clone();
        TransformAttribute2dStepSerde {
            name: self.name().to_string(),
            source0,
            source1,
            target,
            transformer: self.transformer().into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generation::step::assert_eq;
    use omg::data::math::transformer::transformer2d::Transformer2d;

    #[test]
    fn test_conversion() {
        let attributes = vec!["s0".to_string(), "s1".to_string(), "t".to_string()];
        let transformer = Transformer2d::Const(88);
        let step = TransformAttribute2dStep::new("step", 0, 1, 2, transformer).unwrap();

        assert_eq(step, &attributes);
    }
}

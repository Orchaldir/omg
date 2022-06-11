use crate::generation::step::{get_attribute_id, FromStep, ToStep};
use anyhow::{Context, Result};
use omg::generation::attributes::modify::ModifyWithAttributeStep;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ModifyWithAttributeStepSerde {
    source: String,
    target: String,
    percentage: i32,
    minimum: u8,
}

impl ToStep<ModifyWithAttributeStep> for ModifyWithAttributeStepSerde {
    fn try_convert(self, attributes: &mut Vec<String>) -> Result<ModifyWithAttributeStep> {
        let source_id = get_attribute_id(&self.source, attributes)
            .context("Failed to convert source of ModifyWithAttributeStep!")?;
        let target_id = get_attribute_id(&self.target, attributes)
            .context("Failed to convert target of ModifyWithAttributeStep!")?;

        Ok(ModifyWithAttributeStep::new(
            source_id,
            target_id,
            self.percentage,
            self.minimum,
        ))
    }
}

impl FromStep<ModifyWithAttributeStepSerde> for ModifyWithAttributeStep {
    fn convert(&self, attributes: &[String]) -> ModifyWithAttributeStepSerde {
        let source = attributes[self.source_id()].clone();
        let target = attributes[self.target_id()].clone();
        ModifyWithAttributeStepSerde {
            source,
            target,
            percentage: self.percentage(),
            minimum: self.minimum(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generation::step::assert_eq;

    #[test]
    fn test_conversion() {
        let mut attributes = vec!["a".to_string(), "b".to_string()];
        let step = ModifyWithAttributeStep::new(1, 0, 50, 10);

        assert_eq(step, &mut attributes);
    }
}

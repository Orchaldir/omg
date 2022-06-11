use crate::generation::step::{FromStep, ToStep};
use anyhow::{Context, Result};
use omg::generation::attributes::create::CreateAttributeStep;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct CreateAttributeStepSerde {
    attribute: String,
    default: u8,
}

impl ToStep<CreateAttributeStep> for CreateAttributeStepSerde {
    fn try_convert(self, attributes: &mut Vec<String>) -> Result<CreateAttributeStep> {
        attributes.push(self.attribute.clone());

        CreateAttributeStep::new(self.attribute, self.default)
            .context("Failed to convert to CreateAttributeStep!")
    }
}

impl FromStep<CreateAttributeStepSerde> for CreateAttributeStep {
    fn convert(&self, attributes: &mut Vec<String>) -> CreateAttributeStepSerde {
        attributes.push(self.attribute().to_string());

        CreateAttributeStepSerde {
            attribute: self.attribute().to_string(),
            default: self.default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversion() {
        let mut attributes = Vec::new();
        let step = CreateAttributeStep::new("create", 66).unwrap();
        let serde: CreateAttributeStepSerde = (&step).convert(&mut attributes);

        assert_eq!(attributes, vec!["create".to_string()]);
        attributes.clear();

        assert_eq!(serde.try_convert(&mut attributes).unwrap(), step);

        assert_eq!(attributes, vec!["create".to_string()])
    }
}

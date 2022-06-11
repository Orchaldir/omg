use anyhow::{Context, Result};
use omg::generation::attributes::create::CreateAttributeStep;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct CreateAttributeStepSerde {
    attribute: String,
    default: u8,
}

impl CreateAttributeStepSerde {
    pub fn try_convert(self) -> Result<CreateAttributeStep> {
        CreateAttributeStep::new(self.attribute, self.default)
            .context("Failed to convert to CreateAttributeStep!")
    }
}

impl From<&CreateAttributeStep> for CreateAttributeStepSerde {
    fn from(step: &CreateAttributeStep) -> Self {
        CreateAttributeStepSerde {
            attribute: step.attribute().to_string(),
            default: step.default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversion() {
        let start = CreateAttributeStep::new("test", 66).unwrap();
        let serde: CreateAttributeStepSerde = (&start).into();

        assert_eq!(serde.try_convert().unwrap(), start)
    }
}

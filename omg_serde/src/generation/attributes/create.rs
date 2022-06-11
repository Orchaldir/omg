use anyhow::Result;
use omg::generation::attributes::create::CreateAttribute;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct CreateAttributeSerde {
    attribute: String,
    default: u8,
}

impl CreateAttributeSerde {
    pub fn try_convert(self) -> Result<CreateAttribute> {
        CreateAttribute::new(self.attribute, self.default)
    }
}

impl From<&CreateAttribute> for CreateAttributeSerde {
    fn from(step: &CreateAttribute) -> Self {
        CreateAttributeSerde {
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
        let start = CreateAttribute::new("test", 66).unwrap();
        let serde: CreateAttributeSerde = (&start).into();

        assert_eq!(serde.try_convert().unwrap(), start)
    }
}

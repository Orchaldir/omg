use crate::data::map::Map2d;
use crate::data::name::validate_name;
use anyhow::Result;

/// Create a new [`Attribute`](crate::data::map::attribute::Attribute) in the [`Map2d`].
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CreateAttributeStep {
    attribute: String,
    default: u8,
}

impl CreateAttributeStep {
    /// Creates the step, but returns an error if the name is invalid:
    ///
    /// ```
    ///# use omg_core::generation::attributes::create::CreateAttributeStep;
    /// assert!(CreateAttributeStep::new("", 9).is_err());
    /// assert!(CreateAttributeStep::new("   ", 42).is_err());
    /// ```
    pub fn new<S: Into<String>>(attribute: S, default: u8) -> Result<CreateAttributeStep> {
        let name = validate_name(attribute)?;

        Ok(CreateAttributeStep {
            attribute: name,
            default,
        })
    }

    pub fn attribute(&self) -> &str {
        &self.attribute
    }

    pub fn default(&self) -> u8 {
        self.default
    }

    /// Runs the step.
    ///
    /// ```
    ///# use omg_core::data::map::{get_attribute, Map2d};
    ///# use omg_core::data::math::size2d::Size2d;
    ///# use omg_core::generation::attributes::create::CreateAttributeStep;
    /// let size = Size2d::unchecked(2, 3);
    /// let mut map = Map2d::new(size);
    /// let step = CreateAttributeStep::new("test0", 9).unwrap();
    ///
    /// step.run(&mut map);
    ///
    /// let attribute = get_attribute(&map, 0);
    /// assert_eq!(attribute.name(), "test0");
    /// assert_eq!(attribute.size(), &size);
    /// assert_eq!(attribute.get_all(), &vec![9u8, 9, 9, 9, 9, 9]);
    /// ```
    pub fn run(&self, map: &mut Map2d) {
        info!(
            "Create attribute '{}' of map '{}'",
            self.attribute,
            map.name()
        );

        map.create_attribute(self.attribute.clone(), self.default)
            .expect("Failed to create the attribute!");
    }
}

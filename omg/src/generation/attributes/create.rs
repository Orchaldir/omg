use crate::data::map::Map2d;
use crate::data::name::validate_name;
use anyhow::{Context, Result};

/// Create a new [`Attribute`] in the [`Map2d`].
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CreateAttribute {
    attribute: String,
    default: u8,
}

impl CreateAttribute {
    /// Creates the step, but returns an error if the name is invalid:
    ///
    /// ```
    ///# use omg::generation::attributes::create::CreateAttribute;
    /// assert!(CreateAttribute::new("", 9).is_err());
    /// assert!(CreateAttribute::new("   ", 42).is_err());
    /// ```
    pub fn new<S: Into<String>>(name: S, default: u8) -> Result<CreateAttribute> {
        let name = validate_name(name).context("Failed to create a CreateAttribute step!")?;

        Ok(CreateAttribute {
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
    ///# use omg::data::map::Map2d;
    ///# use omg::data::math::size2d::Size2d;
    ///# use omg::generation::attributes::create::CreateAttribute;
    /// let size = Size2d::unchecked(2, 3);
    /// let mut map = Map2d::new(size);
    /// let step = CreateAttribute::new("test0", 9).unwrap();
    ///
    /// step.run(&mut map);
    ///
    /// let attribute = map.get_attribute(0);
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

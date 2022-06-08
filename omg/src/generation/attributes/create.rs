use crate::data::map::Map2d;

/// Create a new [`Attribute`] in the [`Map2d`].
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CreateAttribute {
    name: String,
    default: u8,
}

impl CreateAttribute {
    pub fn new<S: Into<String>>(name: S, default: u8) -> CreateAttribute {
        CreateAttribute {
            name: name.into(),
            default,
        }
    }

    pub fn get_attribute(&self) -> &str {
        &self.name
    }

    /// Runs the step.
    ///
    /// ```
    ///# use omg::data::map::Map2d;
    ///# use omg::data::math::size2d::Size2d;
    ///# use omg::generation::attributes::create::CreateAttribute;
    /// let size = Size2d::unchecked(2, 3);
    /// let mut map = Map2d::new(size);
    /// let step = CreateAttribute::new("test0", 9);
    ///
    /// step.run(&mut map);
    ///
    /// let attribute = map.get_attribute(0);
    /// assert_eq!(attribute.name(), "test0");
    /// assert_eq!(attribute.size(), &size);
    /// assert_eq!(attribute.get_all(), &vec![9u8, 9, 9, 9, 9, 9]);
    /// ```
    pub fn run(&self, map: &mut Map2d) {
        info!("Create attribute '{}' of map '{}'", self.name, map.name());

        map.create_attribute(self.name.clone(), self.default);
    }
}

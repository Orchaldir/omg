use crate::data::map::attribute::Attribute;
use crate::data::math::size2d::Size2d;
use std::collections::HashMap;

pub mod attribute;

/// Represents a 2d region or world map.
pub struct Map2d {
    name: String,
    size: Size2d,
    attribute_lookup: HashMap<String, usize>,
    attributes: Vec<Attribute>,
}

impl Map2d {
    /// Returns a new map.
    pub fn new(size: Size2d) -> Map2d {
        Map2d::with_name("test", size)
    }

    /// Returns a map with a name.
    ///
    /// ```
    ///# use omg::data::map::Map2d;
    ///# use omg::data::math::size2d::Size2d;
    /// let size = Size2d::new(2, 3);
    /// let mut map = Map2d::with_name("world", size);
    ///
    /// assert_eq!(map.name(), "world");
    /// assert_eq!(map.size(), size);
    /// ```
    pub fn with_name<S: Into<String>>(name: S, size: Size2d) -> Map2d {
        Map2d {
            name: name.into(),
            size,
            attribute_lookup: HashMap::new(),
            attributes: Vec::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn size(&self) -> Size2d {
        self.size
    }

    /// Adds a new [`Attribute`] to the map and returns its id.
    ///
    /// ```
    ///# use omg::data::map::Map2d;
    ///# use omg::data::math::size2d::Size2d;
    /// let mut map = Map2d::new(Size2d::new(2, 3));
    ///
    /// assert_eq!(map.create_attribute("elevation", 42), Some(0));
    /// assert_eq!(map.create_attribute("rainfall", 100), Some(1));
    /// ```
    ///
    /// Fails if the map already contains an [`Attribute`] with the same name.
    ///
    /// ```
    ///# use omg::data::map::Map2d;
    ///# use omg::data::math::size2d::Size2d;
    /// let mut map = Map2d::new(Size2d::new(2, 3));
    ///
    /// assert_eq!(map.create_attribute("elevation", 42), Some(0));
    /// assert_eq!(map.create_attribute("elevation", 100), None);
    /// ```
    pub fn create_attribute<S: Into<String>>(&mut self, name: S, default: u8) -> Option<usize> {
        self.add_attribute(Attribute::default_value(name, self.size, default))
    }

    /// Adds a new [`Attribute`] with the supplied values to the map and returns its id.
    pub fn create_attribute_from<S: Into<String>>(
        &mut self,
        name: S,
        values: Vec<u8>,
    ) -> Option<usize> {
        self.add_attribute(Attribute::new(name, self.size, values))
    }

    fn add_attribute(&mut self, attribute: Attribute) -> Option<usize> {
        let id = self.attributes.len();

        if self.attribute_lookup.contains_key(attribute.name()) {
            return None;
        }

        self.attribute_lookup
            .insert(attribute.name().to_string(), id);
        self.attributes.push(attribute);
        Some(id)
    }

    /// Returns the id of the [`Attribute`] with the matching name.
    ///
    /// ```
    ///# use omg::data::map::Map2d;
    ///# use omg::data::math::size2d::Size2d;
    /// let mut map = Map2d::new(Size2d::new(2, 3));
    /// map.create_attribute("elevation", 42);
    /// map.create_attribute("rainfall", 100);
    ///
    /// assert_eq!(map.get_attribute_id("elevation"), Some(0));
    /// assert_eq!(map.get_attribute_id("rainfall"), Some(1));
    /// assert_eq!(map.get_attribute_id("unknown"), None);
    /// ```
    pub fn get_attribute_id(&self, name: &str) -> Option<usize> {
        self.attribute_lookup.get(name).copied()
    }

    /// Returns an [`Attribute`] with the matching id.
    ///
    /// ```
    ///# use omg::data::map::Map2d;
    ///# use omg::data::math::size2d::Size2d;
    /// let mut map = Map2d::new(Size2d::new(2, 3));
    /// map.create_attribute("elevation", 42);
    /// map.create_attribute("rainfall", 100);
    ///
    /// assert_eq!(map.get_attribute(0).name(), "elevation");
    /// assert_eq!(map.get_attribute(1).name(), "rainfall");
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if there is no matching id.
    ///
    /// ```should_panic
    ///# use omg::data::map::Map2d;
    ///# use omg::data::math::size2d::Size2d;
    /// let mut map = Map2d::new(Size2d::new(2, 3));
    ///
    /// map.get_attribute(0);
    /// ```
    pub fn get_attribute(&self, id: usize) -> &Attribute {
        unwrap!(self.attributes.get(id), "Unknown attribute id {}!", id)
    }

    /// Returns a mutable [`Attribute`] with the matching id.
    ///
    /// ```
    ///# use omg::data::map::attribute::Attribute;
    ///# use omg::data::map::Map2d;
    ///# use omg::data::math::size2d::Size2d;
    /// let mut map = Map2d::new(Size2d::new(2, 3));
    /// map.create_attribute("elevation", 42);
    /// map.create_attribute("rainfall", 100);
    ///
    /// assert_eq!(map.get_attribute_mut(0).name(), "elevation");
    /// assert_eq!(map.get_attribute_mut(1).name(), "rainfall");
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if there is no matching id.
    ///
    /// ```should_panic
    ///# use omg::data::map::Map2d;
    ///# use omg::data::math::size2d::Size2d;
    /// let mut map = Map2d::new(Size2d::new(2, 3));
    ///
    /// map.get_attribute_mut(0);
    /// ```
    pub fn get_attribute_mut(&mut self, id: usize) -> &mut Attribute {
        unwrap!(self.attributes.get_mut(id), "Unknown attribute id {}!", id)
    }
}

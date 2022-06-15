use crate::data::map::attribute::Attribute;
use crate::data::math::size2d::Size2d;
use anyhow::{bail, Context, Result};
use std::collections::HashMap;

pub mod attribute;

/// Represents a 2d region or world map.
#[derive(Clone, Debug, PartialEq)]
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
    ///# use omg_core::data::map::Map2d;
    ///# use omg_core::data::math::size2d::Size2d;
    /// let size = Size2d::unchecked(2, 3);
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

    pub fn get_all(&self) -> &[Attribute] {
        &self.attributes
    }

    /// Adds a new [`Attribute`] to the map and returns its id.
    ///
    /// ```
    ///# use omg_core::data::map::Map2d;
    ///# use omg_core::data::math::size2d::Size2d;
    /// let mut map = Map2d::new(Size2d::unchecked(2, 3));
    ///
    /// assert_eq!(map.create_attribute("elevation", 42).unwrap(), 0);
    /// assert_eq!(map.create_attribute("rainfall", 100).unwrap(), 1);
    /// ```
    ///
    /// Fails if the map already contains an [`Attribute`] with the same name.
    ///
    /// ```
    ///# use omg_core::data::map::Map2d;
    ///# use omg_core::data::math::size2d::Size2d;
    /// let mut map = Map2d::new(Size2d::unchecked(2, 3));
    ///
    /// assert_eq!(map.create_attribute("elevation", 42).unwrap(), 0);
    /// assert!(map.create_attribute("elevation", 100).is_err());
    /// ```
    pub fn create_attribute<S: Into<String>>(&mut self, name: S, default: u8) -> Result<usize> {
        self.add_attribute(Attribute::default_value(name, self.size, default))
    }

    /// Adds a new [`Attribute`] with the supplied values to the map and returns its id.
    pub fn create_attribute_from<S: Into<String>>(
        &mut self,
        name: S,
        values: Vec<u8>,
    ) -> Result<usize> {
        self.add_attribute(Attribute::new(name, self.size, values))
    }

    fn add_attribute(&mut self, attribute: Result<Attribute>) -> Result<usize> {
        let attribute = attribute
            .with_context(|| format!("Failed to create attribute for map '{}'!", self.name))?;

        if self.attribute_lookup.contains_key(attribute.name()) {
            bail!(
                "Map '{}' already has an attribute '{}'!",
                self.name,
                attribute.name()
            );
        }

        let id = self.attributes.len();
        self.attribute_lookup
            .insert(attribute.name().to_string(), id);
        self.attributes.push(attribute);
        Ok(id)
    }

    /// Returns the id of the [`Attribute`] with the matching name.
    ///
    /// ```
    ///# use omg_core::data::map::Map2d;
    ///# use omg_core::data::math::size2d::Size2d;
    /// let mut map = Map2d::new(Size2d::unchecked(2, 3));
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

    /// Returns the [`Attribute`] with the matching id.
    ///
    /// ```
    ///# use omg_core::data::map::Map2d;
    ///# use omg_core::data::math::size2d::Size2d;
    /// let mut map = Map2d::new(Size2d::unchecked(2, 3));
    /// map.create_attribute("elevation", 42);
    ///
    /// assert!(map.get_attribute(0).is_some());
    /// assert!(map.get_attribute(1).is_none());
    /// ```
    pub fn get_attribute(&self, id: usize) -> Option<&Attribute> {
        self.attributes.get(id)
    }

    /// Returns a mutable [`Attribute`] with the matching id.
    ///
    /// ```
    ///# use omg_core::data::map::attribute::Attribute;
    ///# use omg_core::data::map::Map2d;
    ///# use omg_core::data::math::size2d::Size2d;
    /// let mut map = Map2d::new(Size2d::unchecked(2, 3));
    /// map.create_attribute("elevation", 42);
    ///
    /// assert!(map.get_attribute_mut(0).is_some());
    /// assert!(map.get_attribute_mut(1).is_none());
    /// ```
    pub fn get_attribute_mut(&mut self, id: usize) -> Option<&mut Attribute> {
        self.attributes.get_mut(id)
    }
}

/// Returns the [`Attribute`] with the matching id.
///
/// ```
///# use omg_core::data::map::{get_attribute, Map2d};
///# use omg_core::data::math::size2d::Size2d;
/// let mut map = Map2d::new(Size2d::unchecked(2, 3));
/// map.create_attribute("elevation", 42);
/// map.create_attribute("rainfall", 100);
///
/// assert_eq!(get_attribute(&map, 0).name(), "elevation");
/// assert_eq!(get_attribute(&map, 1).name(), "rainfall");
/// ```
///
/// # Panics
///
/// Panics if there is no matching id.
///
/// ```should_panic
///# use omg_core::data::map::{get_attribute, Map2d};
///# use omg_core::data::math::size2d::Size2d;
/// let mut map = Map2d::new(Size2d::unchecked(2, 3));
///
/// get_attribute(&map, 0);
/// ```
pub fn get_attribute(map: &Map2d, id: usize) -> &Attribute {
    unwrap!(map.get_attribute(id), "Unknown attribute id {}!", id)
}

/// Returns a mutable [`Attribute`] with the matching id.
///
/// ```
///# use omg_core::data::map::attribute::Attribute;
///# use omg_core::data::map::{get_attribute_mut, Map2d};
///# use omg_core::data::math::size2d::Size2d;
/// let mut map = Map2d::new(Size2d::unchecked(2, 3));
/// map.create_attribute("elevation", 42);
/// map.create_attribute("rainfall", 100);
///
/// assert_eq!(get_attribute_mut(&mut map, 0).name(), "elevation");
/// assert_eq!(get_attribute_mut(&mut map, 1).name(), "rainfall");
/// ```
///
/// # Panics
///
/// Panics if there is no matching id.
///
/// ```should_panic
///# use omg_core::data::map::{get_attribute_mut, Map2d};
///# use omg_core::data::math::size2d::Size2d;
/// let mut map = Map2d::new(Size2d::unchecked(2, 3));
///
/// get_attribute_mut(&mut map, 0);
/// ```
pub fn get_attribute_mut(map: &mut Map2d, id: usize) -> &mut Attribute {
    unwrap!(map.get_attribute_mut(id), "Unknown attribute id {}!", id)
}

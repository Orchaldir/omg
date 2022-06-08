use crate::data::math::size2d::Size2d;
use anyhow::{bail, Result};
use std::ops::{Index, IndexMut};

/// Represents a value with a specific meaning for each cell of a map.
///
/// Examples:
/// * elevation
/// * rainfall
/// * temperature
#[derive(Debug)]
pub struct Attribute {
    name: String,
    size: Size2d,
    values: Vec<u8>,
}

impl Attribute {
    /// Creates an attribute filled with a default value.
    /// See the new constructor for more details.
    pub fn default_value<S: Into<String>>(name: S, size: Size2d, default: u8) -> Result<Attribute> {
        let values = vec![default; size.get_area()];
        Attribute::new(name, size, values)
    }

    /// Creates an attribute from the supplied values, if their number matches the map size.
    ///
    /// ```
    ///# use omg::data::map::attribute::Attribute;
    ///# use omg::data::math::size2d::Size2d;
    /// assert!(Attribute::new("test", Size2d::unchecked(2, 3), vec![0u8, 50]).is_err());
    /// ```
    ///
    /// Also returns an error, if the name is invalid:
    ///
    /// ```
    ///# use omg::data::map::attribute::Attribute;
    ///# use omg::data::math::size2d::Size2d;
    /// let size = Size2d::unchecked(1, 2);
    /// assert!(Attribute::new("", size, vec![0u8, 50]).is_err());
    /// assert!(Attribute::new(" ", size, vec![0u8, 50]).is_err());
    /// ```
    pub fn new<S: Into<String>>(name: S, size: Size2d, values: Vec<u8>) -> Result<Attribute> {
        if size.get_area() != values.len() {
            bail!(
                "The size of the map ({}) doesn't match the number of values ({})!",
                size.get_area(),
                values.len()
            );
        }

        let name = name.into();
        let trimmed = name.trim();

        if trimmed.is_empty() {
            bail!("The attribute name '{}' is invalid!", name);
        }

        Ok(Attribute {
            name: trimmed.to_string(),
            size,
            values,
        })
    }

    /// Returns the name of the attribute.
    ///
    /// ```
    ///# use omg::data::map::attribute::Attribute;
    ///# use omg::data::math::size2d::Size2d;
    /// let attribute = Attribute::default_value("elevation", Size2d::unchecked(2, 3), 42).unwrap();
    ///
    /// assert_eq!(attribute.name(), "elevation");
    /// ```
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the size of the map.
    ///
    /// ```
    ///# use omg::data::map::attribute::Attribute;
    ///# use omg::data::math::size2d::Size2d;
    /// let size = Size2d::unchecked(2, 3);
    /// let attribute = Attribute::default_value("elevation", size, 42).unwrap();
    ///
    /// assert_eq!(attribute.size(), &size);
    /// ```
    pub fn size(&self) -> &Size2d {
        &self.size
    }

    /// Returns a reference to the values.
    ///
    /// ```
    ///# use omg::data::map::attribute::Attribute;
    ///# use omg::data::math::size2d::Size2d;
    /// let attribute = Attribute::new("elevation", Size2d::unchecked(1, 2), vec![10, 15]).unwrap();
    ///
    ///  assert_eq!(attribute.get_all(), &vec![10u8, 15u8]);
    /// ```
    pub fn get_all(&self) -> &Vec<u8> {
        &self.values
    }

    /// Replaces all of the attribute's values.
    ///
    /// ```
    ///# use omg::data::map::attribute::Attribute;
    ///# use omg::data::math::size2d::Size2d;
    /// let mut attribute = Attribute::default_value("elevation", Size2d::unchecked(1, 2), 42).unwrap();
    ///
    /// attribute.replace_all(vec![3, 4]);
    ///
    /// assert_eq!(attribute.get_all(), &vec![3, 4]);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the number of new values is wrong.
    ///
    /// ```should_panic
    ///# use omg::data::map::attribute::Attribute;
    ///# use omg::data::math::size2d::Size2d;
    /// let mut attribute = Attribute::default_value("elevation", Size2d::unchecked(1, 2), 42).unwrap();
    ///
    /// attribute.replace_all(vec![3, 4, 5]);
    /// ```
    pub fn replace_all(&mut self, values: Vec<u8>) {
        assert_eq!(
            values.len(),
            self.values.len(),
            "Wrong number of new values!"
        );
        self.values = values;
    }

    /// Replaces some of the attribute's values.
    ///
    /// ```
    ///# use omg::data::map::attribute::Attribute;
    ///# use omg::data::math::size2d::Size2d;
    /// let mut attribute = Attribute::default_value("elevation", Size2d::unchecked(1, 3), 42).unwrap();
    ///
    /// attribute.replace_some(vec![0, 2], 5);
    ///
    /// assert_eq!(attribute.get_all(), &vec![5u8, 42, 5]);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if an index is outside th map.
    ///
    /// ```should_panic
    ///# use omg::data::map::attribute::Attribute;
    ///# use omg::data::math::size2d::Size2d;
    /// let mut attribute = Attribute::default_value("elevation", Size2d::unchecked(1, 2), 42).unwrap();
    ///
    /// attribute.replace_some(vec![5], 9);
    /// ```
    pub fn replace_some(&mut self, indices: Vec<usize>, value: u8) {
        for index in indices.iter() {
            self.values[*index] = value;
        }
    }
}

/// Returns the value at the index.
///
/// ```
///# use omg::data::map::attribute::Attribute;
///# use omg::data::math::size2d::Size2d;
/// let attribute = Attribute::new("elevation", Size2d::unchecked(1, 2), vec![6, 7]).unwrap();
///
/// assert_eq!(attribute[0], 6);
/// assert_eq!(attribute[1], 7);
/// ```
///
/// # Panics
///
/// Panics if the index is outside the map.
///
/// ```should_panic
///# use omg::data::map::attribute::Attribute;
///# use omg::data::math::size2d::Size2d;
/// let attribute = Attribute::default_value("elevation", Size2d::unchecked(1, 2), 42).unwrap();
///
/// attribute[2];
/// ```
impl Index<usize> for Attribute {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

/// Returns the mutable value at the index.
///
/// ```
///# use omg::data::map::attribute::Attribute;
///# use omg::data::math::size2d::Size2d;
/// let mut attribute = Attribute::default_value("elevation", Size2d::unchecked(1, 2), 42).unwrap();
///
/// attribute[0] += 4;
///
/// assert_eq!(attribute.get_all(), &vec![46, 42]);
/// ```
///
/// # Panics
///
/// Panics if the index is outside the .
///
/// ```should_panic
///# use omg::data::map::attribute::Attribute;
///# use omg::data::math::size2d::Size2d;
/// let mut attribute = Attribute::default_value("elevation", Size2d::unchecked(1, 2), 42).unwrap();
///
/// attribute[2] = 99;
/// ```
impl IndexMut<usize> for Attribute {
    fn index_mut(&mut self, index: usize) -> &mut u8 {
        &mut self.values[index]
    }
}

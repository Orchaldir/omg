use crate::data::math::size2d::Size2d;

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
    /// Returns an attribute filled with a default value.
    pub fn default_value<S: Into<String>>(name: S, size: Size2d, default: u8) -> Attribute {
        let values = vec![default; size.get_area()];
        Attribute::new(name, size, values)
    }

    /// Returns an attribute from the supplied values.
    pub fn new<S: Into<String>>(name: S, size: Size2d, values: Vec<u8>) -> Attribute {
        assert_eq!(size.get_area(), values.len());
        Attribute {
            name: name.into(),
            size,
            values,
        }
    }

    /// Returns the name of the attribute.
    ///
    /// ```
    ///# use omg::data::map::attribute::Attribute;
    ///# use omg::data::math::size2d::Size2d;
    /// let attribute = Attribute::default_value("elevation", Size2d::new(2, 3), 42);
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
    /// let size = Size2d::new(2, 3);
    /// let attribute = Attribute::default_value("elevation", size, 42);
    ///
    /// assert_eq!(attribute.size(), &size);
    /// ```
    pub fn size(&self) -> &Size2d {
        &self.size
    }

    /// Returns the value at the index.
    ///
    /// ```
    ///# use omg::data::map::attribute::Attribute;
    ///# use omg::data::math::size2d::Size2d;
    /// let attribute = Attribute::default_value("elevation", Size2d::new(1, 2), 42);
    ///
    /// assert_eq!(attribute.get(0), 42);
    /// assert_eq!(attribute.get(1), 42);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the index is outside the map.
    ///
    /// ```should_panic
    ///# use omg::data::map::attribute::Attribute;
    ///# use omg::data::math::size2d::Size2d;
    /// let attribute = Attribute::default_value("elevation", Size2d::new(1, 2), 42);
    ///
    /// attribute.get(2);
    /// ```
    pub fn get(&self, index: usize) -> u8 {
        self.values[index]
    }

    /// Returns the mutable value at the index.
    ///
    /// ```
    ///# use omg::data::map::attribute::Attribute;
    ///# use omg::data::math::size2d::Size2d;
    /// let mut attribute = Attribute::default_value("elevation", Size2d::new(1, 2), 42);
    ///
    /// *attribute.get_mut(0) += 4;
    ///
    /// assert_eq!(attribute.get(0), 46);
    /// assert_eq!(attribute.get(1), 42);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the index is outside the map.
    ///
    /// ```should_panic
    ///# use omg::data::map::attribute::Attribute;
    ///# use omg::data::math::size2d::Size2d;
    /// let mut attribute = Attribute::default_value("elevation", Size2d::new(1, 2), 42);
    ///
    /// attribute.get_mut(2);
    /// ```
    pub fn get_mut(&mut self, index: usize) -> &mut u8 {
        self.values.get_mut(index).expect("Index is outside map!")
    }

    /// Returns a reference to the values.
    ///
    /// ```
    ///# use omg::data::map::attribute::Attribute;
    ///# use omg::data::math::size2d::Size2d;
    /// let attribute = Attribute::new("elevation", Size2d::new(1, 2), vec![10, 15]);
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
    /// let mut attribute = Attribute::default_value("elevation", Size2d::new(1, 2), 42);
    ///
    /// attribute.replace_all(vec![3, 4]);
    ///
    /// assert_eq!(attribute.get(0), 3);
    /// assert_eq!(attribute.get(1), 4);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the number of new values is wrong.
    ///
    /// ```should_panic
    ///# use omg::data::map::attribute::Attribute;
    ///# use omg::data::math::size2d::Size2d;
    /// let mut attribute = Attribute::default_value("elevation", Size2d::new(1, 2), 42);
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
    /// let mut attribute = Attribute::default_value("elevation", Size2d::new(1, 3), 42);
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
    /// let mut attribute = Attribute::default_value("elevation", Size2d::new(1, 2), 42);
    ///
    /// attribute.replace_some(vec![5], 9);
    /// ```
    pub fn replace_some(&mut self, indices: Vec<usize>, value: u8) {
        for index in indices.iter() {
            self.values[*index] = value;
        }
    }
}

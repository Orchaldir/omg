use crate::data::map::{get_attribute, Map2d};
use crate::data::math::generator::generator2d::Generator2d;
use crate::data::name::validate_name;
use anyhow::Result;

/// Modifies an [`Attribute`] with the values generated by a [`Generator2d`].
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GeneratorStep {
    name: String,
    attribute_id: usize,
    generator: Generator2d,
}

impl GeneratorStep {
    pub fn new<S: Into<String>>(
        name: S,
        attribute_id: usize,
        generator: Generator2d,
    ) -> Result<GeneratorStep> {
        let name = validate_name(name)?;

        Ok(GeneratorStep {
            name,
            attribute_id,
            generator,
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn attribute_id(&self) -> usize {
        self.attribute_id
    }

    pub fn generator(&self) -> &Generator2d {
        &self.generator
    }

    // Adds the values.
    ///
    /// ```
    ///# use omg::data::map::{get_attribute, Map2d};
    ///# use omg::data::math::generator::generator2d::Generator2d::IndexGenerator;
    ///# use omg::data::math::size2d::Size2d;
    ///# use omg::generation::attributes::generator::GeneratorStep;
    ///# use omg::generation::step::GenerationStep;
    /// let size = Size2d::unchecked(2, 3);
    /// let mut map = Map2d::new(size);
    /// let attribute_id = map.create_attribute("elevation", 40).unwrap();
    /// let generator = IndexGenerator(size);
    /// let step = GeneratorStep::new("test", attribute_id,generator).unwrap();
    ///
    /// step.add(&mut map);
    ///
    /// let attribute = get_attribute(&map, attribute_id);
    /// assert_eq!(attribute.get_all(), &vec![40u8, 41, 42, 43, 44, 45]);
    /// ```
    pub fn add(&self, map: &mut Map2d) {
        info!(
            "Add '{}' to attribute '{}' of map '{}'",
            self.name,
            get_attribute(map, self.attribute_id).name(),
            map.name()
        );

        let size = map.size();
        let attribute = map.get_attribute_mut(self.attribute_id);
        let mut index = 0;

        for y in 0..size.height() {
            for x in 0..size.width() {
                let value = self.generator.generate(x, y);
                let sum = attribute[index].saturating_add(value);
                attribute[index] = sum;
                index += 1;
            }
        }
    }

    // Subtracts the values.
    ///
    /// ```
    ///# use omg::data::map::{get_attribute, Map2d};
    ///# use omg::data::math::generator::generator2d::Generator2d::IndexGenerator;
    ///# use omg::data::math::size2d::Size2d;
    ///# use omg::generation::attributes::generator::GeneratorStep;
    ///# use omg::generation::step::GenerationStep;
    /// let size = Size2d::unchecked(2, 3);
    /// let mut map = Map2d::new(size);
    /// let attribute_id = map.create_attribute("elevation", 40).unwrap();
    /// let generator = IndexGenerator(size);
    /// let step = GeneratorStep::new("test", attribute_id, generator).unwrap();
    ///
    /// step.sub(&mut map);
    ///
    /// let attribute = get_attribute(&map, attribute_id);
    /// assert_eq!(attribute.get_all(), &vec![40u8, 39, 38, 37, 36, 35]);
    /// ```
    pub fn sub(&self, map: &mut Map2d) {
        info!(
            "Subtract '{}' from attribute '{}' of map '{}'",
            self.name,
            get_attribute(map, self.attribute_id).name(),
            map.name()
        );

        let size = map.size();
        let attribute = map.get_attribute_mut(self.attribute_id);
        let mut index = 0;

        for y in 0..size.height() {
            for x in 0..size.width() {
                let value = self.generator.generate(x, y);
                let sum = attribute[index].saturating_sub(value);
                attribute[index] = sum;
                index += 1;
            }
        }
    }
}

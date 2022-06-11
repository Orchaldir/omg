use crate::data::map::attribute::Attribute;
use crate::data::map::{get_attribute, Map2d};
use crate::data::math::generator::generator1d::Generator1d;

/// Shifts each column or row of an [`Attribute`] based on a [`Generator1d`].
#[derive(new, Debug, PartialEq, Eq, Clone)]
pub struct Distortion1dStep {
    attribute_id: usize,
    generator: Generator1d,
}

impl Distortion1dStep {
    pub fn attribute_id(&self) -> usize {
        self.attribute_id
    }

    pub fn generator(&self) -> &Generator1d {
        &self.generator
    }

    /// Shifts each each row along the x-axis based on a [`Generator1d`].
    ///
    /// ```
    ///# use omg::data::map::{get_attribute, Map2d};
    ///# use omg::data::math::generator::generator1d::Generator1d::InputAsOutput;
    ///# use omg::data::math::size2d::Size2d;
    ///# use omg::generation::attributes::distortion1d::Distortion1dStep;
    /// let size = Size2d::unchecked(3, 3);
    /// let mut map = Map2d::new(size);
    /// let values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    /// let attribute_id = map.create_attribute_from("test", values).unwrap();
    /// let step = Distortion1dStep::new(attribute_id, InputAsOutput);
    ///
    /// step.distort_along_x(&mut map);
    ///
    /// let attribute = get_attribute(&map, attribute_id);
    /// assert_eq!(attribute.get_all(), &vec![1u8, 2, 3, 4, 4, 5, 7, 7, 7]);
    /// ```
    pub fn distort_along_x(&self, map: &mut Map2d) {
        info!(
            "Distort attribute '{}' of map '{}' along the x-axis.",
            get_attribute(map, self.attribute_id).name(),
            map.name()
        );

        let values = self.distort_map_along_x(map);
        let attribute = map.get_attribute_mut(self.attribute_id);

        attribute.replace_all(values);
    }

    /// Shifts each each column along the y-axis based on a [`Generator1d`].
    ///
    /// ```
    ///# use omg::data::map::{get_attribute, Map2d};
    ///# use omg::data::math::generator::generator1d::Generator1d::InputAsOutput;
    ///# use omg::data::math::size2d::Size2d;
    ///# use omg::generation::attributes::distortion1d::Distortion1dStep;
    /// let size = Size2d::unchecked(3, 3);
    /// let mut map = Map2d::new(size);
    /// let values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    /// let attribute_id = map.create_attribute_from("test", values).unwrap();
    /// let step = Distortion1dStep::new(attribute_id, InputAsOutput);
    ///
    /// step.distort_along_y(&mut map);
    ///
    /// let attribute = get_attribute(&map, attribute_id);
    /// assert_eq!(attribute.get_all(), &vec![1u8, 2, 3, 4, 2, 3, 7, 5, 3]);
    /// ```
    pub fn distort_along_y(&self, map: &mut Map2d) {
        info!(
            "Distort attribute '{}' of map '{}' along the y-axis.",
            get_attribute(map, self.attribute_id).name(),
            map.name()
        );

        let values = self.distort_map_along_y(map);
        let attribute = map.get_attribute_mut(self.attribute_id);

        attribute.replace_all(values);
    }

    fn distort_map_along_x(&self, map: &Map2d) -> Vec<u8> {
        let length = map.size().get_area();
        let attribute = get_attribute(map, self.attribute_id);
        let mut values = Vec::with_capacity(length);

        for y in 0..map.size().height() {
            let shift = self.generator.generate(y);
            self.distort_row(y, shift, attribute, &mut values);
        }

        values
    }

    fn distort_map_along_y(&self, map: &Map2d) -> Vec<u8> {
        let length = map.size().get_area();
        let attribute = get_attribute(map, self.attribute_id);
        let mut values = vec![0; length];

        for x in 0..map.size().width() {
            let shift = self.generator.generate(x);
            self.distort_column(x, shift, attribute, &mut values);
        }

        values
    }

    fn distort_row(&self, y: u32, shift: u8, attribute: &Attribute, values: &mut Vec<u8>) {
        let start = attribute.size().to_index_risky(0, y);
        let start_value = attribute[start];

        for _x in 0..shift {
            values.push(start_value);
        }

        let width = attribute.size().width().saturating_sub(shift as u32) as usize;

        for x in 0..width {
            values.push(attribute[start + x]);
        }
    }

    fn distort_column(&self, x: u32, shift: u8, attribute: &Attribute, values: &mut [u8]) {
        let start = attribute.size().to_index_risky(x, 0);
        let start_value = attribute[start];
        let mut index = start;
        let width = attribute.size().width() as usize;

        for _y in 0..shift {
            values[index] = start_value;
            index += width;
        }

        let remaining_height = attribute.size().height().saturating_sub(shift as u32);
        let mut distorted_index = start;

        for _y in 0..remaining_height {
            values[index] = attribute[distorted_index];
            index += width;
            distorted_index += width;
        }
    }
}

use crate::data::map::{get_attribute, get_attribute_mut, Map2d};
use crate::data::math::generator::generator2d::Generator2d;

/// Distorts an [`Attribute`] along 2 dimensions.
#[derive(new, Debug, PartialEq, Eq, Clone)]
pub struct Distortion2dStep {
    attribute_id: usize,
    generator_x: Generator2d,
    generator_y: Generator2d,
}

impl Distortion2dStep {
    pub fn attribute_id(&self) -> usize {
        self.attribute_id
    }

    pub fn generator_x(&self) -> &Generator2d {
        &self.generator_x
    }

    pub fn generator_y(&self) -> &Generator2d {
        &self.generator_y
    }

    // Runs the step.
    pub fn run(&self, map: &mut Map2d) {
        info!(
            "Distort attribute '{}' of map '{}' in 2 dimensions.",
            get_attribute(map, self.attribute_id).name(),
            map.name()
        );

        let values = self.distort_map(map);
        let attribute = get_attribute_mut(map, self.attribute_id);

        attribute.replace_all(values);
    }

    fn distort_map(&self, map: &Map2d) -> Vec<u8> {
        let size = map.size();
        let length = size.get_area();
        let attribute = get_attribute(map, self.attribute_id);
        let mut values = Vec::with_capacity(length);

        for y in 0..size.height() {
            for x in 0..size.width() {
                let shift_x = self.generator_x.generate(x, y) as u32;
                let shift_y = self.generator_y.generate(x, y) as u32;
                let distorted_x = x + shift_x;
                let distorted_y = y + shift_y;
                let index = size.saturating_to_index(distorted_x, distorted_y);
                values.push(attribute[index]);
            }
        }

        values
    }
}

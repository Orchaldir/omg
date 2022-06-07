use crate::data::map::Map2d;

/// Modifies one [`Attribute`] with another transformed one.
#[derive(new, Debug, Clone)]
pub struct ModifyWithAttribute {
    source_id: usize,
    target_id: usize,
    factor: f32,
    minimum: u8,
}

impl ModifyWithAttribute {
    // Runs the step.
    pub fn run(&self, map: &mut Map2d) {
        info!(
            "{} attribute '{}' with attribute '{}' of map '{}'",
            if self.factor < 0.0 {
                "Decrease"
            } else {
                "Increase"
            },
            map.get_attribute(self.target_id).name(),
            map.get_attribute(self.source_id).name(),
            map.name()
        );

        let values = self.calculate_values(map);
        let attribute = map.get_attribute_mut(self.target_id);

        attribute.replace_all(values);
    }

    fn calculate_value(&self, source: u8, target: u8) -> u8 {
        (target as f32 + (source.max(self.minimum) - self.minimum) as f32 * self.factor) as u8
    }

    fn calculate_values(&self, map: &mut Map2d) -> Vec<u8> {
        let length = map.size().get_area();
        let source_attribute = map.get_attribute(self.source_id);
        let target_attribute = map.get_attribute(self.target_id);
        let mut values = Vec::with_capacity(length);

        for index in 0..length {
            let source = source_attribute[index];
            let target = target_attribute[index];
            values.push(self.calculate_value(source, target));
        }

        values
    }
}

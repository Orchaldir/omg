use crate::data::map::{get_attribute, Map2d};

/// Modifies one [`Attribute`] with another transformed one.
#[derive(new, Debug, PartialEq, Eq, Clone)]
pub struct ModifyWithAttributeStep {
    source_id: usize,
    target_id: usize,
    percentage: i32,
    minimum: u8,
}

impl ModifyWithAttributeStep {
    pub fn source_id(&self) -> usize {
        self.source_id
    }

    pub fn target_id(&self) -> usize {
        self.target_id
    }

    pub fn percentage(&self) -> i32 {
        self.percentage
    }

    pub fn minimum(&self) -> u8 {
        self.minimum
    }

    // Runs the step.
    pub fn run(&self, map: &mut Map2d) {
        let factor = self.percentage as f32 / 100.0;
        info!(
            "{} attribute '{}' with attribute '{}' of map '{}'",
            if factor < 0.0 { "Decrease" } else { "Increase" },
            get_attribute(map, self.target_id).name(),
            get_attribute(map, self.source_id).name(),
            map.name()
        );

        let values = self.calculate_values(map, factor);
        let attribute = map.get_attribute_mut(self.target_id);

        attribute.replace_all(values);
    }

    fn calculate_values(&self, map: &mut Map2d, factor: f32) -> Vec<u8> {
        let length = map.size().get_area();
        let source_attribute = get_attribute(map, self.source_id);
        let target_attribute = get_attribute(map, self.target_id);
        let mut values = Vec::with_capacity(length);

        for index in 0..length {
            let source = source_attribute[index];
            let target = target_attribute[index];
            values.push(self.calculate_value(source, target, factor));
        }

        values
    }

    fn calculate_value(&self, source: u8, target: u8, factor: f32) -> u8 {
        (target as f32 + (source.max(self.minimum) - self.minimum) as f32 * factor) as u8
    }
}

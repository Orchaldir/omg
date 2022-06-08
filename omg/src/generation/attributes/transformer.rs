use crate::data::map::Map2d;
use crate::data::math::transformer::transformer2d::Transformer2d;

/// Transforms 2 [`Attribute`]s and writes into another.
#[derive(new)]
pub struct TransformAttribute2d {
    name: String,
    source_id0: usize,
    source_id1: usize,
    target_id: usize,
    transformer: Transformer2d,
}

impl TransformAttribute2d {
    // Runs the step.
    ///
    /// ```
    ///# use omg::data::map::Map2d;
    ///# use omg::data::math::size2d::Size2d;
    ///# use omg::data::math::transformer::transformer2d::Transformer2d;
    ///# use omg::generation::attributes::transformer::TransformAttribute2d;
    /// let mut map = Map2d::new(Size2d::unchecked(3, 2));
    /// map.create_attribute_from("input0", vec![  0,   1,  99, 100, 101, 255]);
    /// map.create_attribute_from("input1", vec![200, 199, 198, 197, 196, 195]);
    /// map.create_attribute("target", 10);
    /// let transformer = Transformer2d::new_overwrite_if_below(42, 100);
    /// let step = TransformAttribute2d::new("name".to_string(), 0, 1, 2, transformer);
    ///
    /// step.run(&mut map);
    ///
    /// assert_eq!(map.get_attribute(0).get_all(), &vec![  0,   1,  99, 100, 101, 255]);
    /// assert_eq!(map.get_attribute(1).get_all(), &vec![200, 199, 198, 197, 196, 195]);
    /// assert_eq!(map.get_attribute(2).get_all(), &vec![ 42,  42,  42,  42, 196, 195]);
    /// ```
    pub fn run(&self, map: &mut Map2d) {
        info!(
            "Apply transformation '{}' using '{}' & '{}' to '{}' of map '{}'",
            self.name,
            map.get_attribute(self.source_id0).name(),
            map.get_attribute(self.source_id1).name(),
            map.get_attribute(self.target_id).name(),
            map.name()
        );

        let biomes = self.transform(map);
        let attribute = map.get_attribute_mut(self.target_id);

        attribute.replace_all(biomes);
    }

    fn transform(&self, map: &mut Map2d) -> Vec<u8> {
        let size = map.size();
        let source_attribute0 = map.get_attribute(self.source_id0);
        let source_attribute1 = map.get_attribute(self.source_id1);
        let mut biomes = Vec::with_capacity(size.get_area());

        for index in 0..size.get_area() {
            let value0 = source_attribute0[index];
            let value1 = source_attribute1[index];
            biomes.push(self.transformer.transform(value0, value1));
        }

        biomes
    }
}

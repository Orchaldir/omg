use crate::data::map::{get_attribute, get_attribute_mut, Map2d};
use crate::data::math::transformer::transformer2d::Transformer2d;
use crate::data::name::validate_name;
use anyhow::{bail, Result};

/// Transforms 2 [`Attribute`]s and writes into another.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TransformAttribute2dStep {
    name: String,
    source_id0: usize,
    source_id1: usize,
    target_id: usize,
    transformer: Transformer2d,
}

impl TransformAttribute2dStep {
    pub fn new<S: Into<String>>(
        name: S,
        source_id0: usize,
        source_id1: usize,
        target_id: usize,
        transformer: Transformer2d,
    ) -> Result<TransformAttribute2dStep> {
        let name = validate_name(name)?;

        if source_id0 == source_id1 {
            bail!("Both source ids are {}!", source_id0);
        }

        Ok(TransformAttribute2dStep {
            name,
            source_id0,
            source_id1,
            target_id,
            transformer,
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn source_id0(&self) -> usize {
        self.source_id0
    }

    pub fn source_id1(&self) -> usize {
        self.source_id1
    }

    pub fn target_id(&self) -> usize {
        self.target_id
    }

    pub fn transformer(&self) -> &Transformer2d {
        &self.transformer
    }

    // Runs the step.
    ///
    /// ```
    ///# use omg_core::data::map::{get_attribute, Map2d};
    ///# use omg_core::data::math::size2d::Size2d;
    ///# use omg_core::data::math::transformer::transformer2d::Transformer2d;
    ///# use omg_core::generation::attributes::transformer::TransformAttribute2dStep;
    /// let mut map = Map2d::new(Size2d::unchecked(3, 2));
    /// map.create_attribute_from("input0", vec![  0,   1,  99, 100, 101, 255]);
    /// map.create_attribute_from("input1", vec![200, 199, 198, 197, 196, 195]);
    /// map.create_attribute("target", 10);
    /// let transformer = Transformer2d::new_overwrite_if_below(42, 100);
    /// let step = TransformAttribute2dStep::new("name".to_string(), 0, 1, 2, transformer).unwrap();
    ///
    /// step.run(&mut map);
    ///
    /// assert_eq!(get_attribute(&map, 0).get_all(), &vec![  0,   1,  99, 100, 101, 255]);
    /// assert_eq!(get_attribute(&map, 1).get_all(), &vec![200, 199, 198, 197, 196, 195]);
    /// assert_eq!(get_attribute(&map, 2).get_all(), &vec![ 42,  42,  42,  42, 196, 195]);
    /// ```
    pub fn run(&self, map: &mut Map2d) {
        info!(
            "Apply transformation '{}' using '{}' & '{}' to '{}' of map '{}'",
            self.name,
            get_attribute(map, self.source_id0).name(),
            get_attribute(map, self.source_id1).name(),
            get_attribute(map, self.target_id).name(),
            map.name()
        );

        let biomes = self.transform(map);
        let attribute = get_attribute_mut(map, self.target_id);

        attribute.replace_all(biomes);
    }

    fn transform(&self, map: &mut Map2d) -> Vec<u8> {
        let size = map.size();
        let source_attribute0 = get_attribute(map, self.source_id0);
        let source_attribute1 = get_attribute(map, self.source_id1);
        let mut biomes = Vec::with_capacity(size.get_area());

        for index in 0..size.get_area() {
            let value0 = source_attribute0[index];
            let value1 = source_attribute1[index];
            biomes.push(self.transformer.transform(value0, value1));
        }

        biomes
    }
}

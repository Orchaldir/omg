use omg_core::data::map::Map2d;
use omg_core::data::math::selector::ColorSelector;
use omg_core::interface::selector::SelectorStorage;
use omg_serde::interface::selector::SelectorStorageWithSerde;
use std::collections::HashMap;

pub fn init_selectors(map: &Map2d) -> HashMap<usize, ColorSelector> {
    let selector_storage = SelectorStorageWithSerde::new();

    map.get_all()
        .iter()
        .enumerate()
        .filter_map(|(i, attribute)| {
            selector_storage
                .read(&format!(
                    "../resources/color_selector/{}.yaml",
                    attribute.name()
                ))
                .map(|s| (i, s))
                .ok()
        })
        .collect()
}

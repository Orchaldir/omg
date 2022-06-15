use omg_core::data::color::*;
use omg_core::data::math::selector::ColorSelector;
use omg_core::data::math::size2d::Size2d;
use omg_core::generation::step::GenerationStep;
use omg_core::generation::MapGeneration;
use omg_core::interface::map::MapStorage;
use omg_core::interface::selector::SelectorStorage;
use omg_serde::interface::map::MapStorageWithSerde;
use omg_serde::interface::selector::SelectorStorageWithSerde;

#[test]
fn test_map_storage() {
    let storage = MapStorageWithSerde {};

    let size = Size2d::unchecked(1, 2);
    let steps = vec![GenerationStep::debug("a"), GenerationStep::debug("b")];
    let map_generation = MapGeneration::new("test0", size, steps).unwrap();

    let path = "map.yaml";

    storage.write(&map_generation, path).unwrap();

    let result = storage.read(path).unwrap();

    assert_eq!(result, map_generation);
}

#[test]
fn test_color_selector_storage() {
    let storage = SelectorStorageWithSerde {};

    let selector = ColorSelector::new_interpolate_vector(vec![
        (0u8, WHITE),
        (51, CYAN),
        (102, BLUE),
        (153, GREEN),
        (204, YELLOW),
        (255, RED),
    ])
    .unwrap();

    let path = "color_selector.yaml";

    storage.write(&selector, path).unwrap();

    let result = storage.read(path).unwrap();

    assert_eq!(result, selector);
}

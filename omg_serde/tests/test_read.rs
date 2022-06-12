use omg::interface::map::MapStorage;
use omg_serde::interface::map::MapStorageWithSerde;

#[test]
fn test_read() {
    let port = MapStorageWithSerde {};
    port.read(&"../resources/map_generation/biome.yaml")
        .unwrap();
}

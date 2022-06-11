use omg::interface::io::StoragePort;
use omg_serde::interface::io::StoragePortWithSerde;

#[test]
fn test_read() {
    let port = StoragePortWithSerde {};
    port.read(&"../resources/map_generation/biome.yaml")
        .unwrap();
}

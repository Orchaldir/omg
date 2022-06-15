use crate::generation::MapGenerator;
use anyhow::Result;

/// This is an interface to save & load maps generators.
/// It keeps additional dependencies like [serde](https://serde.rs) out of the core repo.
pub trait MapStorage {
    fn read(&self, path: &str) -> Result<MapGenerator>;

    fn write(&self, map_generator: &MapGenerator, path: &str) -> Result<()>;
}

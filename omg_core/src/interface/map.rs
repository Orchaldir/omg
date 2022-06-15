use crate::generation::MapGeneration;
use anyhow::Result;

/// This is an interface to save & load maps generators.
/// It keeps additional dependencies like [serde](https://serde.rs) out of the core repo.
pub trait MapStorage {
    fn read(&self, path: &str) -> Result<MapGeneration>;

    fn write(&self, map_generator: &MapGeneration, path: &str) -> Result<()>;
}

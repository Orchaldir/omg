use crate::generation::MapGeneration;
use anyhow::Result;

pub trait MapStorage {
    fn read(&self, path: &str) -> Result<MapGeneration>;

    fn write(&self, map_generator: &MapGeneration, path: &str) -> Result<()>;
}

use crate::generation::MapGenerationSerde;
use anyhow::{Context, Result};
use omg::generation::MapGeneration;
use omg::interface::map::MapStorage;
use std::fs;
use std::fs::File;
use std::io::Write;

pub struct MapStorageWithSerde;

impl MapStorageWithSerde {
    pub fn inner_read(&self, path: &str) -> Result<MapGeneration> {
        let string = fs::read_to_string(path)?;
        let data: MapGenerationSerde = serde_yaml::from_str(&string)?;
        data.try_convert()
    }

    pub fn inner_write(&self, map_generator: &MapGeneration, path: &str) -> Result<()> {
        let mut file = File::create(path)?;

        let data: MapGenerationSerde = map_generator.into();
        let s = serde_yaml::to_string(&data)?;

        file.write_all(s.as_bytes())?;

        Ok(())
    }
}

impl MapStorage for MapStorageWithSerde {
    fn read(&self, path: &str) -> Result<MapGeneration> {
        self.inner_read(path)
            .with_context(|| format!("Failed to read MapGeneration from '{}'", path))
    }

    fn write(&self, map_generator: &MapGeneration, path: &str) -> Result<()> {
        self.inner_write(map_generator, path).with_context(|| {
            format!(
                "Failed to write MapGeneration '{}' to '{}'",
                map_generator.name(),
                path
            )
        })
    }
}
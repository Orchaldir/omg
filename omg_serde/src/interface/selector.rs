use anyhow::{Context, Result};
use omg::interface::selector::{ColorSelector, SelectorStorage};
use std::fs;
use std::fs::File;
use std::io::Write;
use crate::data::color::ColorSerde;
use crate::data::math::selector::SelectorSerde;

pub type ColorSelectorSerde = SelectorSerde<u8,ColorSerde>;

pub struct SelectorStorageWithSerde;

impl SelectorStorageWithSerde {
    pub fn inner_read(&self, path: &str) -> Result<ColorSelector> {
        let string = fs::read_to_string(path)?;
        let data: ColorSelectorSerde = serde_yaml::from_str(&string)?;
        Ok(data.into())
    }

    pub fn inner_write(&self, selector: &ColorSelector, path: &str) -> Result<()> {
        let mut file = File::create(path)?;

        let data: ColorSelectorSerde = selector.into();
        let s = serde_yaml::to_string(&data)?;

        file.write_all(s.as_bytes())?;

        Ok(())
    }
}

impl SelectorStorage for SelectorStorageWithSerde {
    fn read(&self, path: &str) -> Result<ColorSelector> {
        self.inner_read(path)
            .with_context(|| format!("Failed to read selector from '{}'", path))
    }

    fn write(&self, selector: &ColorSelector, path: &str) -> Result<()> {
        self.inner_write(selector, path).with_context(|| {
            format!(
                "Failed to write selector to '{}'",
                path
            )
        })
    }
}
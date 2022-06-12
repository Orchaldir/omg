use crate::data::math::selector::ColorSelector;
use anyhow::Result;

pub trait SelectorStorage {
    fn read(&self, path: &str) -> Result<ColorSelector>;

    fn write(&self, selector: &ColorSelector, path: &str) -> Result<()>;
}

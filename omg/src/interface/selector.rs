use crate::data::color::Color;
use crate::data::math::selector::Selector;
use anyhow::Result;

pub type ColorSelector = Selector<u8, Color>;

pub trait SelectorStorage {
    fn read(&self, path: &str) -> Result<ColorSelector>;

    fn write(&self, selector: &ColorSelector, path: &str) -> Result<()>;
}

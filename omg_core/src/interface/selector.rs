use crate::data::math::selector::ColorSelector;
use anyhow::Result;

/// This is an interface to save & load color selectors.
/// It keeps additional dependencies like [serde](https://serde.rs) out of the core repo.
pub trait SelectorStorage {
    fn read(&self, path: &str) -> Result<ColorSelector>;

    fn write(&self, selector: &ColorSelector, path: &str) -> Result<()>;
}

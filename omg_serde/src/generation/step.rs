use anyhow::{Context, Result};
use std::fmt::Debug;

pub fn get_attribute_id(attribute: &str, attributes: &[String]) -> Result<usize> {
    attributes
        .iter()
        .position(|name| name.eq(attribute))
        .with_context(|| format!("Unknown attribute '{}'", attribute))
}

pub trait ToStep<T> {
    fn try_convert(self, attributes: &[String]) -> Result<T>;
}

pub trait FromStep<T> {
    fn convert(&self, attributes: &[String]) -> T;
}

pub fn assert_eq<R: FromStep<S> + Eq + Debug, S: ToStep<R>>(step: R, attributes: &[String]) {
    let serde: S = (&step).convert(attributes);

    assert_eq!(serde.try_convert(attributes).unwrap(), step)
}

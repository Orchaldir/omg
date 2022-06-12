use anyhow::Result;

pub trait TryConvert<T> {
    fn try_convert(self) -> Result<T>;
}

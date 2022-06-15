use anyhow::{bail, Result};

pub fn validate_name<S: Into<String>>(name: S) -> Result<String> {
    let name = name.into();
    let trimmed = name.trim();

    if trimmed.is_empty() {
        bail!("The name '{}' is invalid!", name);
    }

    Ok(trimmed.to_string())
}

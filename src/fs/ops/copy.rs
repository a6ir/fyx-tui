use std::path::Path;

use anyhow::{anyhow, Result};

pub fn copy_path(_from: &Path, _to: &Path) -> Result<()> {
    Err(anyhow!("copy is not implemented in this iteration"))
}

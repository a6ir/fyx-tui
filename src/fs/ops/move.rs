use std::{fs as std_fs, path::Path};

use anyhow::Result;

pub fn move_path(from: &Path, to: &Path) -> Result<()> {
    std_fs::rename(from, to)?;
    Ok(())
}

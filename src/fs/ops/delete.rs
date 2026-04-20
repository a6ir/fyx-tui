use std::{fs as std_fs, path::Path};

use anyhow::Result;

pub fn delete_path(path: &Path) -> Result<()> {
    if path.is_dir() {
        std_fs::remove_dir_all(path)?;
    } else {
        std_fs::remove_file(path)?;
    }

    Ok(())
}

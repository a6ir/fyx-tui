use std::{fs as std_fs, path::Path};

use anyhow::Result;

use crate::fs::{entry::FsEntry, sort};

pub fn read_entries(path: &Path) -> Result<Vec<FsEntry>> {
    let mut entries = Vec::new();

    for item in std_fs::read_dir(path)? {
        let Ok(dir_entry) = item else {
            continue;
        };

        let file_type = match dir_entry.file_type() {
            Ok(file_type) => file_type,
            Err(_) => continue,
        };

        let name = dir_entry.file_name().to_string_lossy().to_string();
        if name.is_empty() {
            continue;
        }

        entries.push(FsEntry {
            path: dir_entry.path(),
            name,
            is_dir: file_type.is_dir(),
            size: dir_entry.metadata().map(|m| m.len()).unwrap_or(0),
            modified: dir_entry.metadata().and_then(|m| m.modified()).ok(),
        });
    }

    sort::sort_entries(&mut entries);
    Ok(entries)
}

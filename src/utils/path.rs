use std::path::{Path, PathBuf};

pub fn parent_or_self(path: &Path) -> PathBuf {
    path.parent()
        .map_or_else(|| path.to_path_buf(), |parent| parent.to_path_buf())
}

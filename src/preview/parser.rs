use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone)]
pub enum PreviewKind {
    Missing,
    Directory(PathBuf),
    Text(PathBuf),
    Binary(PathBuf),
}

pub fn detect(path: &Path) -> PreviewKind {
    if !path.exists() {
        return PreviewKind::Missing;
    }

    if path.is_dir() {
        return PreviewKind::Directory(path.to_path_buf());
    }

    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(_) => return PreviewKind::Binary(path.to_path_buf()),
    };

    let mut buffer = [0_u8; 1024];
    let read = match file.read(&mut buffer) {
        Ok(count) => count,
        Err(_) => return PreviewKind::Binary(path.to_path_buf()),
    };

    if buffer[..read].contains(&0) {
        PreviewKind::Binary(path.to_path_buf())
    } else {
        PreviewKind::Text(path.to_path_buf())
    }
}

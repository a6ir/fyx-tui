use std::{fs as std_fs, path::Path};

use crate::preview::parser::{self, PreviewKind};

const MAX_TEXT_LINES: usize = 80;

pub fn render_preview(path: &Path) -> String {
    match parser::detect(path) {
        PreviewKind::Missing => String::from("[missing]") ,
        PreviewKind::Directory(dir) => render_directory_preview(&dir),
        PreviewKind::Text(file) => render_text_preview(&file),
        PreviewKind::Binary(file) => render_metadata_preview(&file),
    }
}

fn render_directory_preview(path: &Path) -> String {
    let mut lines = Vec::new();
    lines.push(format!("dir: {}", path.display()));

    let read_dir = match std_fs::read_dir(path) {
        Ok(entries) => entries,
        Err(_) => {
            lines.push(String::from("[unreadable directory]"));
            return lines.join("\n");
        }
    };

    for (index, entry) in read_dir.enumerate() {
        if index >= MAX_TEXT_LINES {
            break;
        }

        let Ok(entry) = entry else {
            continue;
        };

        let name = entry.file_name().to_string_lossy().to_string();
        lines.push(name);
    }

    if lines.len() == 1 {
        lines.push(String::from("[empty directory]"));
    }

    lines.join("\n")
}

fn render_text_preview(path: &Path) -> String {
    match std_fs::read_to_string(path) {
        Ok(contents) => contents
            .lines()
            .take(MAX_TEXT_LINES)
            .collect::<Vec<_>>()
            .join("\n"),
        Err(_) => render_metadata_preview(path),
    }
}

fn render_metadata_preview(path: &Path) -> String {
    match std_fs::metadata(path) {
        Ok(metadata) => {
            let kind = if metadata.is_dir() { "directory" } else { "binary" };
            format!(
                "{}\n\nsize: {} bytes\nreadonly: {}",
                kind,
                metadata.len(),
                metadata.permissions().readonly()
            )
        }
        Err(_) => String::from("[binary or unreadable]"),
    }
}

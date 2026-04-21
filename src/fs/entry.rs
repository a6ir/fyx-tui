use std::path::PathBuf;
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct FsEntry {
    pub path: PathBuf,
    pub name: String,
    pub is_dir: bool,
    pub modified: Option<SystemTime>,
    pub size: u64,
}

impl FsEntry {
    pub fn display_name(&self) -> String {
        if self.is_dir {
            format!("{}/", self.name)
        } else {
            self.name.clone()
        }
    }

    pub fn display_size(&self) -> String {
        if self.is_dir {
            return String::from("--");
        }

        let kb = self.size as f64 / 1024.0;
        if kb < 1.0 {
            format!("{} B", self.size)
        } else if kb < 1024.0 {
            format!("{:.1} KB", kb)
        } else {
            format!("{:.1} MB", kb / 1024.0)
        }
    }

    pub fn display_date(&self) -> String {
        match self.modified {
            Some(time) => {
                let datetime: chrono::DateTime<chrono::Local> = time.into();
                datetime.format("%Y-%m-%d %H:%M").to_string()
            }
            None => String::from("--"),
        }
    }
}

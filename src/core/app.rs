use std::{path::PathBuf, time::Instant};

use crate::{core::mode::Mode, fs::entry::FsEntry};

#[derive(Clone)]
pub struct ClickState {
    pub row: u16,
    pub at: Instant,
}

pub struct App {
    pub cwd: PathBuf,
    pub parent: Vec<FsEntry>,
    pub current: Vec<FsEntry>,
    pub preview: String,

    pub selected: usize,
    pub scroll: usize,

    pub mode: Mode,
    pub status: String,

    pub command_input: String,
    pub search_input: String,

    pub pending_g: bool,
    pub running: bool,
    pub last_click: Option<ClickState>,

    pub preview_token: u64,
}

impl App {
    pub fn new(cwd: PathBuf) -> Self {
        Self {
            cwd,
            parent: Vec::new(),
            current: Vec::new(),
            preview: String::new(),
            selected: 0,
            scroll: 0,
            mode: Mode::Normal,
            status: String::from("ready"),
            command_input: String::new(),
            search_input: String::new(),
            pending_g: false,
            running: true,
            last_click: None,
            preview_token: 0,
        }
    }
}

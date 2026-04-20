use std::{path::PathBuf, time::Instant};

use crate::{core::mode::Mode, fs::entry::FsEntry};

#[derive(Clone)]
pub struct ClickState {
    pub row: u16,
    pub at: Instant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaneFocus {
    Left,
    Right,
}

pub struct App {
    pub cwd: PathBuf,
    pub parent: Vec<FsEntry>,
    pub current: Vec<FsEntry>,
    pub full_entries: Vec<FsEntry>,
    pub filtered: Vec<FsEntry>,
    pub preview: String,

    pub selected: usize,
    pub scroll: usize,

    pub mode: Mode,
    pub focus: PaneFocus,
    pub status: String,

    pub search_query: String,
    pub command_buffer: String,

    pub pending_g: bool,
    pub running: bool,
    pub last_click: Option<ClickState>,

    pub preview_token: u64,
    pub preview_enabled: bool,
}

impl App {
    pub fn new(cwd: PathBuf) -> Self {
        Self {
            cwd,
            parent: Vec::new(),
            current: Vec::new(),
            full_entries: Vec::new(),
            filtered: Vec::new(),
            preview: String::new(),
            selected: 0,
            scroll: 0,
            mode: Mode::Normal,
            focus: PaneFocus::Left,
            status: String::from("ready"),
            search_query: String::new(),
            command_buffer: String::new(),
            pending_g: false,
            running: true,
            last_click: None,
            preview_token: 0,
            preview_enabled: true,
        }
    }
}

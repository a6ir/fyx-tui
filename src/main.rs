mod commands;
mod config;
mod core;
mod event;
mod fs;
mod input;
mod preview;
mod search;
mod ui;
mod utils;

use anyhow::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

use crate::core::{app::App, context::AppContext};

struct TerminalGuard;

impl TerminalGuard {
    fn setup() -> Result<Self> {
        enable_raw_mode()?;
        execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;
        Ok(Self)
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture);
    }
}

fn main() -> Result<()> {
    let _guard = TerminalGuard::setup()?;

    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    let cwd = std::env::current_dir()?;
    let mut app = App::new(cwd);

    let (preview_req_tx, preview_res_rx) = preview::worker::start_worker();
    let ctx = AppContext {
        preview_req_tx,
        preview_res_rx,
    };

    let result = event::r#loop::run_app(&mut terminal, &mut app, &ctx);

    terminal.show_cursor()?;
    result
}

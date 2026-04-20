use anyhow::Result;
use crossterm::event::Event as CrosstermEvent;
use ratatui::layout::Rect;

use crate::{
    core::{app::App, context::AppContext, mode::Mode, state},
    input::{keymap, mouse},
    ui::layout,
};

pub fn handle_input(
    app: &mut App,
    ctx: &AppContext,
    event: CrosstermEvent,
    area: Rect,
) -> Result<()> {
    let ui_layout = layout::split(area, matches!(app.mode, Mode::Command | Mode::Search));

    match event {
        CrosstermEvent::Key(key) => {
            let action = keymap::map_key(key);
            match app.mode {
                Mode::Normal => handle_normal_mode(app, ctx, action)?,
                Mode::Search => handle_search_mode(app, action),
                Mode::Command => handle_command_mode(app, action),
            }
        }
        CrosstermEvent::Mouse(mouse_event) => {
            mouse::handle_mouse(app, ctx, mouse_event, ui_layout.current)?;
        }
        CrosstermEvent::Resize(_, _) => {}
        _ => {}
    }

    let viewport_rows = ui_layout.current.height.saturating_sub(2) as usize;
    state::sync_scroll(app, viewport_rows);
    Ok(())
}

fn handle_normal_mode(app: &mut App, ctx: &AppContext, action: keymap::KeyAction) -> Result<()> {
    let mut refresh_preview = false;

    match action {
        keymap::KeyAction::MoveDown => {
            app.pending_g = false;
            state::move_selection(app, 1);
            refresh_preview = true;
        }
        keymap::KeyAction::MoveUp => {
            app.pending_g = false;
            state::move_selection(app, -1);
            refresh_preview = true;
        }
        keymap::KeyAction::Enter => {
            app.pending_g = false;
            if state::enter_selected(app)? {
                app.status = format!("entered {}", app.cwd.display());
                refresh_preview = true;
            }
        }
        keymap::KeyAction::Parent => {
            app.pending_g = false;
            if state::go_parent(app)? {
                app.status = format!("parent {}", app.cwd.display());
                refresh_preview = true;
            }
        }
        keymap::KeyAction::JumpBottom => {
            app.pending_g = false;
            state::jump_bottom(app);
            refresh_preview = true;
        }
        keymap::KeyAction::Char('g') => {
            if app.pending_g {
                state::jump_top(app);
                refresh_preview = true;
                app.pending_g = false;
            } else {
                app.pending_g = true;
            }
        }
        keymap::KeyAction::StartSearch => {
            app.pending_g = false;
            app.mode = Mode::Search;
            app.search_input.clear();
        }
        keymap::KeyAction::StartCommand => {
            app.pending_g = false;
            app.mode = Mode::Command;
            app.command_input.clear();
        }
        keymap::KeyAction::Quit => {
            app.running = false;
        }
        _ => {
            app.pending_g = false;
        }
    }

    if refresh_preview {
        state::request_preview(app, ctx);
    }

    Ok(())
}

fn handle_search_mode(app: &mut App, action: keymap::KeyAction) {
    match action {
        keymap::KeyAction::Escape => {
            app.mode = Mode::Normal;
        }
        keymap::KeyAction::Backspace => {
            app.search_input.pop();
        }
        keymap::KeyAction::Char(ch) => {
            app.search_input.push(ch);
        }
        keymap::KeyAction::Submit => {
            app.mode = Mode::Normal;
            app.status = format!("search queued: {}", app.search_input);
        }
        _ => {}
    }
}

fn handle_command_mode(app: &mut App, action: keymap::KeyAction) {
    match action {
        keymap::KeyAction::Escape => {
            app.mode = Mode::Normal;
        }
        keymap::KeyAction::Backspace => {
            app.command_input.pop();
        }
        keymap::KeyAction::Char(ch) => {
            app.command_input.push(ch);
        }
        keymap::KeyAction::Submit => {
            app.mode = Mode::Normal;
            app.status = format!("command queued: {}", app.command_input);
        }
        _ => {}
    }
}

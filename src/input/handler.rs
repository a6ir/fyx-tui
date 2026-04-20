use anyhow::Result;
use crossterm::event::Event as CrosstermEvent;
use ratatui::layout::Rect;

use crate::{
    commands,
    core::{
        app::{App, PaneFocus},
        context::AppContext,
        mode::Mode,
        state,
    },
    input::{keymap, mouse},
    search,
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
                Mode::Search => handle_search_mode(app, ctx, action),
                Mode::Command => handle_command_mode(app, ctx, action)?,
            }
        }
        CrosstermEvent::Mouse(mouse_event) => {
            if app.focus == PaneFocus::Left {
                mouse::handle_mouse(app, ctx, mouse_event, ui_layout.left)?;
            }
        }
        CrosstermEvent::Resize(_, _) => {}
        _ => {}
    }

    let viewport_rows = ui_layout.left.height.saturating_sub(2) as usize;
    state::sync_scroll(app, viewport_rows);
    Ok(())
}

fn handle_normal_mode(app: &mut App, ctx: &AppContext, action: keymap::KeyAction) -> Result<()> {
    match action {
        keymap::KeyAction::SwitchFocus => {
            app.focus = match app.focus {
                PaneFocus::Left => PaneFocus::Right,
                PaneFocus::Right => PaneFocus::Left,
            };
            return Ok(());
        }
        keymap::KeyAction::Char('p') => {
            toggle_preview(app, ctx);
            return Ok(());
        }
        keymap::KeyAction::StartSearch => {
            app.pending_g = false;
            search::state::enter_search_mode(app);
            state::request_preview(app, ctx);
            return Ok(());
        }
        keymap::KeyAction::StartCommand => {
            app.pending_g = false;
            app.mode = Mode::Command;
            app.command_buffer.clear();
            return Ok(());
        }
        keymap::KeyAction::Char('q') => {
            app.running = false;
            return Ok(());
        }
        _ => {}
    }

    match app.focus {
        PaneFocus::Left => handle_left_pane_normal_mode(app, ctx, action),
        PaneFocus::Right => handle_right_pane_normal_mode(app, action),
    }
}

fn handle_left_pane_normal_mode(
    app: &mut App,
    ctx: &AppContext,
    action: keymap::KeyAction,
) -> Result<()> {
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
        _ => {
            app.pending_g = false;
        }
    }

    if refresh_preview {
        state::request_preview(app, ctx);
    }

    Ok(())
}

fn handle_right_pane_normal_mode(app: &mut App, action: keymap::KeyAction) -> Result<()> {
    if matches!(action, keymap::KeyAction::Parent) {
        app.status = String::from("Switch to left pane (Tab) for navigation");
    }

    app.pending_g = false;
    Ok(())
}

fn handle_search_mode(app: &mut App, ctx: &AppContext, action: keymap::KeyAction) {
    let mut refresh_preview = false;

    match action {
        keymap::KeyAction::Escape => {
            search::state::cancel_search(app);
            refresh_preview = true;
        }
        keymap::KeyAction::Backspace => {
            search::state::backspace(app);
            refresh_preview = true;
        }
        keymap::KeyAction::Char(ch) => {
            search::state::push_char(app, ch);
            refresh_preview = true;
        }
        keymap::KeyAction::Submit => {
            search::state::submit_search(app);
            refresh_preview = true;
        }
        _ => {}
    }

    if refresh_preview {
        state::request_preview(app, ctx);
    }
}

fn handle_command_mode(app: &mut App, ctx: &AppContext, action: keymap::KeyAction) -> Result<()> {
    match action {
        keymap::KeyAction::Escape => {
            app.mode = Mode::Normal;
            app.command_buffer.clear();
        }
        keymap::KeyAction::Backspace => {
            app.command_buffer.pop();
        }
        keymap::KeyAction::Char(ch) => {
            app.command_buffer.push(ch);
        }
        keymap::KeyAction::Submit => {
            let parsed = commands::parser::parse_command(&app.command_buffer);
            commands::executor::execute(app, ctx, parsed)?;
            app.mode = Mode::Normal;
            app.command_buffer.clear();
        }
        _ => {}
    }

    Ok(())
}

fn toggle_preview(app: &mut App, ctx: &AppContext) {
    app.preview_enabled = !app.preview_enabled;

    if app.preview_enabled {
        state::request_preview(app, ctx);
        app.status = String::from("preview enabled");
    } else {
        app.preview = String::from("Preview disabled. Press 'p' to enable.");
        app.status = String::from("preview disabled");
    }
}

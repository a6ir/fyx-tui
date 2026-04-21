use std::path::PathBuf;

use anyhow::Result;
use crossterm::event::{Event as CrosstermEvent, KeyCode, KeyEventKind, MouseEventKind};
use ratatui::layout::Rect;

use crate::{
    commands,
    core::{
        app::{App, PaneFocus},
        context::AppContext,
        mode::Mode,
        state,
    },
    input::mouse,
    search,
    ui::{layout, panes::shortcuts},
};

pub fn handle_input(
    app: &mut App,
    ctx: &AppContext,
    event: CrosstermEvent,
    area: Rect,
) -> Result<()> {
    let ui_layout = layout::split(area);

    match event {
        CrosstermEvent::Key(key) if key.kind == KeyEventKind::Press => match app.mode {
            Mode::Normal => handle_normal_mode(app, ctx, key.code)?,
            Mode::Search => handle_search_mode(app, ctx, key.code),
            Mode::Command => handle_command_mode(app, ctx, key.code)?,
        },
        CrosstermEvent::Mouse(mouse_event) => {
            if matches!(mouse_event.kind, MouseEventKind::Down(_)) {
                if point_in_rect(mouse_event.column, mouse_event.row, ui_layout.shortcuts) {
                    app.focus = PaneFocus::Shortcuts;
                } else if point_in_rect(mouse_event.column, mouse_event.row, ui_layout.current) {
                    app.focus = PaneFocus::Current;
                }
            }

            if app.focus == PaneFocus::Current {
                mouse::handle_mouse(app, ctx, mouse_event, ui_layout.current)?;
            }
        }
        CrosstermEvent::Resize(_, _) => {}
        _ => {}
    }

    let viewport_rows = ui_layout.current.height.saturating_sub(2) as usize;
    state::sync_scroll(app, viewport_rows);
    Ok(())
}

fn handle_normal_mode(app: &mut App, ctx: &AppContext, code: KeyCode) -> Result<()> {
    match code {
        KeyCode::Tab => {
            app.focus = match app.focus {
                PaneFocus::Shortcuts => PaneFocus::Current,
                PaneFocus::Current => PaneFocus::Shortcuts,
            };
        }
        KeyCode::Char('p') => toggle_preview(app, ctx),
        KeyCode::Char('/') => {
            search::state::enter_search_mode(app);
            state::request_preview(app, ctx);
        }
        KeyCode::Char(':') => {
            app.mode = Mode::Command;
            app.command_buffer.clear();
        }
        KeyCode::Char('q') => app.running = false,
        _ => match app.focus {
            PaneFocus::Current => handle_current_pane_normal(app, ctx, code)?,
            PaneFocus::Shortcuts => handle_shortcuts_pane_normal(app, ctx, code)?,
        },
    }

    Ok(())
}

fn handle_current_pane_normal(app: &mut App, ctx: &AppContext, code: KeyCode) -> Result<()> {
    let mut refresh_preview = false;

    match code {
        KeyCode::Char('j') => {
            app.pending_g = false;
            app.is_scrolling = true;
            state::move_selection(app, 1);
            refresh_preview = true;
        }
        KeyCode::Char('k') => {
            app.pending_g = false;
            app.is_scrolling = true;
            state::move_selection(app, -1);
            refresh_preview = true;
        }
        KeyCode::Char('l') | KeyCode::Enter => {
            app.pending_g = false;
            if state::enter_selected(app)? {
                app.status = format!("entered {}", app.cwd.display());
                app.last_previewed = None;
                refresh_preview = true;
            }
        }
        KeyCode::Char('h') => {
            app.pending_g = false;
            if state::go_parent(app)? {
                app.status = format!("parent {}", app.cwd.display());
                app.last_previewed = None;
                refresh_preview = true;
            }
        }
        KeyCode::Char('G') => {
            app.pending_g = false;
            app.is_scrolling = true;
            state::jump_bottom(app);
            refresh_preview = true;
        }
        KeyCode::Char('g') => {
            if app.pending_g {
                app.is_scrolling = true;
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

fn handle_shortcuts_pane_normal(app: &mut App, ctx: &AppContext, code: KeyCode) -> Result<()> {
    let max_index = shortcuts::shortcuts().len().saturating_sub(1);

    match code {
        KeyCode::Char('j') => {
            app.shortcuts_selected = (app.shortcuts_selected + 1).min(max_index);
        }
        KeyCode::Char('k') => {
            app.shortcuts_selected = app.shortcuts_selected.saturating_sub(1);
        }
        KeyCode::Char('l') | KeyCode::Enter => {
            if let Some(target) = shortcut_target(app.shortcuts_selected) {
                if target.exists() && target.is_dir() {
                    app.cwd = target;
                    state::refresh_directory(app)?;
                    app.last_previewed = None;
                    state::request_preview(app, ctx);
                    app.status = format!("entered {}", app.cwd.display());
                    app.focus = PaneFocus::Current;
                } else {
                    app.status = String::from("shortcut path unavailable");
                }
            }
        }
        KeyCode::Char('h') => {
            if state::go_parent(app)? {
                app.last_previewed = None;
                state::request_preview(app, ctx);
                app.status = format!("parent {}", app.cwd.display());
            }
        }
        _ => {}
    }

    Ok(())
}

fn handle_search_mode(app: &mut App, ctx: &AppContext, code: KeyCode) {
    let mut refresh_preview = false;

    match code {
        KeyCode::Esc => {
            search::state::cancel_search(app);
            refresh_preview = true;
        }
        KeyCode::Backspace => {
            search::state::backspace(app);
            refresh_preview = true;
        }
        KeyCode::Enter => {
            search::state::submit_search(app);
            refresh_preview = true;
        }
        KeyCode::Char(ch) => {
            search::state::push_char(app, ch);
            refresh_preview = true;
        }
        _ => {}
    }

    if refresh_preview {
        app.last_previewed = None;
        state::request_preview(app, ctx);
    }
}

fn handle_command_mode(app: &mut App, ctx: &AppContext, code: KeyCode) -> Result<()> {
    match code {
        KeyCode::Esc => {
            app.mode = Mode::Normal;
            app.command_buffer.clear();
        }
        KeyCode::Backspace => {
            app.command_buffer.pop();
        }
        KeyCode::Enter => {
            let parsed = commands::parser::parse_command(&app.command_buffer);
            commands::executor::execute(app, ctx, parsed)?;
            app.mode = Mode::Normal;
            app.command_buffer.clear();
        }
        KeyCode::Char(ch) => {
            app.command_buffer.push(ch);
        }
        _ => {}
    }

    Ok(())
}

fn toggle_preview(app: &mut App, ctx: &AppContext) {
    app.preview_enabled = !app.preview_enabled;

    if app.preview_enabled {
        app.last_previewed = None;
        state::request_preview(app, ctx);
        app.status = String::from("preview enabled");
    } else {
        app.preview = String::from("Preview disabled. Press 'p' to enable.");
        app.last_previewed = None;
        app.status = String::from("preview disabled");
    }
}

fn shortcut_target(index: usize) -> Option<PathBuf> {
    let home = std::env::var("HOME").ok().map(PathBuf::from)?;
    shortcuts::shortcut_path(index, &home)
}

fn point_in_rect(x: u16, y: u16, rect: Rect) -> bool {
    x >= rect.x && x < rect.x + rect.width && y >= rect.y && y < rect.y + rect.height
}

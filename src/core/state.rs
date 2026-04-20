use std::path::PathBuf;

use anyhow::Result;

use crate::{
    core::{app::App, context::AppContext},
    fs::{dir, entry::FsEntry},
    preview::worker::PreviewRequest,
};

pub fn refresh_directory(app: &mut App) -> Result<()> {
    app.current = dir::read_entries(&app.cwd)?;

    app.parent = if let Some(parent) = app.cwd.parent() {
        dir::read_entries(parent).unwrap_or_default()
    } else {
        Vec::new()
    };

    clamp_selection(app);
    app.scroll = 0;
    Ok(())
}

pub fn move_selection(app: &mut App, delta: isize) {
    let len = app.current.len();
    if len == 0 {
        app.selected = 0;
        return;
    }

    let max_index = (len - 1) as isize;
    let next = (app.selected as isize + delta).clamp(0, max_index);
    app.selected = next as usize;
}

pub fn jump_top(app: &mut App) {
    app.selected = 0;
}

pub fn jump_bottom(app: &mut App) {
    if app.current.is_empty() {
        app.selected = 0;
    } else {
        app.selected = app.current.len() - 1;
    }
}

pub fn enter_selected(app: &mut App) -> Result<bool> {
    let Some(entry) = selected_entry(app) else {
        return Ok(false);
    };

    if !entry.is_dir {
        return Ok(false);
    }

    app.cwd = entry.path.clone();
    app.selected = 0;
    app.scroll = 0;
    refresh_directory(app)?;
    Ok(true)
}

pub fn go_parent(app: &mut App) -> Result<bool> {
    let Some(parent) = app.cwd.parent() else {
        return Ok(false);
    };

    app.cwd = parent.to_path_buf();
    app.selected = 0;
    app.scroll = 0;
    refresh_directory(app)?;
    Ok(true)
}

pub fn selected_entry(app: &App) -> Option<&FsEntry> {
    app.current.get(app.selected)
}

pub fn selected_path(app: &App) -> Option<PathBuf> {
    selected_entry(app).map(|entry| entry.path.clone())
}

pub fn request_preview(app: &mut App, ctx: &AppContext) {
    let Some(path) = selected_path(app) else {
        app.preview = String::from("(empty directory)");
        return;
    };

    app.preview_token = app.preview_token.wrapping_add(1);
    let _ = ctx.preview_req_tx.send(PreviewRequest {
        token: app.preview_token,
        path,
    });
}

pub fn drain_preview(app: &mut App, ctx: &AppContext) {
    while let Ok(message) = ctx.preview_res_rx.try_recv() {
        if message.token == app.preview_token {
            app.preview = message.content;
        }
    }
}

pub fn sync_scroll(app: &mut App, viewport_rows: usize) {
    if viewport_rows == 0 {
        return;
    }

    if app.selected < app.scroll {
        app.scroll = app.selected;
    } else if app.selected >= app.scroll + viewport_rows {
        app.scroll = app.selected + 1 - viewport_rows;
    }
}

fn clamp_selection(app: &mut App) {
    if app.current.is_empty() {
        app.selected = 0;
    } else if app.selected >= app.current.len() {
        app.selected = app.current.len() - 1;
    }
}

use std::path::PathBuf;

use ratatui::{
    layout::Rect,
    text::Line,
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};

use crate::{
    core::app::{App, PaneFocus},
    ui::theme,
};

const SHORTCUTS: [&str; 3] = ["Home", "Documents", "Downloads"];

pub fn shortcuts() -> &'static [&'static str] {
    &SHORTCUTS
}

pub fn shortcut_path(index: usize, home: &PathBuf) -> Option<PathBuf> {
    match index {
        0 => Some(home.clone()),
        1 => Some(home.join("Documents")),
        2 => Some(home.join("Downloads")),
        _ => None,
    }
}

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let items: Vec<ListItem> = SHORTCUTS
        .iter()
        .map(|entry| ListItem::new(Line::raw(*entry)))
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(if app.focus == PaneFocus::Shortcuts {
                    theme::active_border()
                } else {
                    theme::border()
                })
                .title("Shortcuts"),
        )
        .highlight_style(theme::selection())
        .highlight_symbol("  ");

    let mut state = ListState::default();
    state.select(Some(app.shortcuts_selected.min(SHORTCUTS.len().saturating_sub(1))));

    frame.render_stateful_widget(list, area, &mut state);
}

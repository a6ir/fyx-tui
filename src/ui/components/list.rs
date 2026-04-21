use ratatui::{
    layout::Rect,
    text::Line,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};

use crate::{fs::entry::FsEntry, ui::theme};

pub fn render(
    frame: &mut Frame,
    area: Rect,
    title: &str,
    entries: &[FsEntry],
    selected: Option<usize>,
    scroll: usize,
    dim_content: bool,
    active: bool,
) {
    let border_style = if active {
        theme::active_border()
    } else {
        theme::border()
    };

    if entries.is_empty() {
        let empty = Paragraph::new("(empty)")
            .wrap(Wrap { trim: true })
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(border_style)
                    .title(title),
            );
        frame.render_widget(empty, area);
        return;
    }

    let items: Vec<ListItem> = entries
        .iter()
        .map(|entry| {
            let line = if dim_content {
                Line::styled(entry.display_name(), theme::dim())
            } else {
                Line::raw(entry.display_name())
            };
            ListItem::new(line)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(border_style)
                .title(title),
        )
        // Applies the background color/bolding to the selected item.
        .highlight_style(theme::selection())
        .highlight_symbol("  ");

    let mut state = ListState::default();
    state.select(selected);
    state = state.with_offset(scroll);

    frame.render_stateful_widget(list, area, &mut state);
}
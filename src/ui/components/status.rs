use ratatui::{
    layout::Rect,
    text::Line,
    widgets::{Block, Paragraph},
    Frame,
};

use crate::{
    core::{app::App, mode::Mode},
    ui::theme,
};

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let line = match app.mode {
        Mode::Normal => format!(" {} | {} | {} ", app.cwd.display(), app.status, app.filtered.len()),
        Mode::Search => format!(" /{} | {} matches ", app.search_query, app.filtered.len()),
        Mode::Command => format!(" :{} ", app.command_buffer),
    };

    let status = Paragraph::new(Line::raw(line))
        .style(theme::status())
        .block(Block::default());
    frame.render_widget(status, area);
}

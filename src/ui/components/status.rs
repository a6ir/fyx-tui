use ratatui::{
    layout::Rect,
    text::Line,
    widgets::{Block, Paragraph},
    Frame,
};

use crate::{core::app::App, ui::theme};

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let line = format!(" {} | {} | {} ", app.cwd.display(), app.status, app.current.len());
    let status = Paragraph::new(Line::raw(line))
        .style(theme::status())
        .block(Block::default());
    frame.render_widget(status, area);
}

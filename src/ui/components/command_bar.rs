use ratatui::{
    layout::Rect,
    text::Line,
    widgets::{Block, Paragraph},
    Frame,
};

use crate::core::{app::App, mode::Mode};

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let text = match app.mode {
        Mode::Command => format!(":{}", app.command_buffer),
        Mode::Search => format!("/{}", app.search_query),
        Mode::Normal => String::new(),
    };

    let bar = Paragraph::new(Line::raw(text)).block(Block::default());
    frame.render_widget(bar, area);
}

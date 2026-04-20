use ratatui::{
    layout::Rect,
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::{core::app::App, ui::theme};

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let paragraph = Paragraph::new(app.preview.as_str())
        .wrap(Wrap { trim: false })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(theme::border())
                .title("Preview"),
        );

    frame.render_widget(paragraph, area);
}

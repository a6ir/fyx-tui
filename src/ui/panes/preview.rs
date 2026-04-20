use ratatui::{
    layout::Rect,
    text::Line,
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::{
    core::app::{App, PaneFocus},
    ui::theme,
};

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let content = if app.preview_enabled {
        app.preview.as_str()
    } else {
        "Preview disabled. Press 'p' to enable."
    };

    let paragraph = Paragraph::new(Line::raw(content))
        .wrap(Wrap { trim: false })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(if app.focus == PaneFocus::Right {
                    theme::active_border()
                } else {
                    theme::border()
                })
                .title("Secondary"),
        );

    frame.render_widget(paragraph, area);
}

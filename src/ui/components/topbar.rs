use ratatui::{
    layout::Rect,
    text::Line,
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::{
    core::{app::App, mode::Mode},
    ui::theme,
};

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let text = match app.mode {
        Mode::Normal => String::from("/ Search... + Command"),
        Mode::Search => format!("/{}", app.search_query),
        Mode::Command => format!(":{}", app.command_buffer),
    };

    let bar = Paragraph::new(Line::raw(text))
        .wrap(Wrap { trim: true })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(theme::border())
                .title("Input"),
        );

    frame.render_widget(bar, area);
}

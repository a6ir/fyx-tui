use ratatui::{
    layout::Rect,
    text::Line,
    widgets::{Block, Paragraph, Wrap},
    Frame,
};

use crate::{
    core::{app::App, mode::Mode},
    ui::theme,
};

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let mode = match app.mode {
        Mode::Normal => "normal",
        Mode::Search => "search",
        Mode::Command => "command",
    };

    let line = format!(
        " {} | items:{} | mode:{} | {} ",
        app.cwd.display(),
        app.filtered.len(),
        mode,
        app.status
    );

    let status = Paragraph::new(Line::raw(line))
        .wrap(Wrap { trim: true })
        .style(theme::status())
        .block(Block::default());
    frame.render_widget(status, area);
}

use ratatui::style::{Color, Modifier, Style};

pub fn border() -> Style {
    Style::default().fg(Color::DarkGray)
}

pub fn selection() -> Style {
    Style::default()
        .fg(Color::Black)
        .bg(Color::Gray)
        .add_modifier(Modifier::BOLD)
}

pub fn dim() -> Style {
    Style::default().fg(Color::Gray)
}

pub fn status() -> Style {
    Style::default().fg(Color::White).bg(Color::DarkGray)
}

use ratatui::style::{Color, Modifier, Style};

/// Defines the style for standard, inactive borders (e.g., unfocused windows).
pub fn border() -> Style {
    // TO MODIFY: Change `Color::DarkGray` to any preset like `Color::White` or `Color::Blue`.
    Style::default().fg(Color::DarkGray)
}

/// Defines the style for the active, focused element's border.
pub fn active_border() -> Style {
    // TO MODIFY: Currently uses a custom RGB color (a lime green). 
    // You can change the RGB values, or replace it with a standard color like `Color::Green` or `Color::Yellow`.
    Style::default().fg(Color::Rgb(163, 230, 53))
}

/// Defines the style for the currently selected item in lists or menus.
pub fn selection() -> Style {
    Style::default()
        // TO MODIFY TEXT COLOR: Change `Color::Black`
        .fg(Color::Black)
        // TO MODIFY HIGHLIGHT COLOR: Change `Color::Gray` to your preferred highlight color (e.g., `Color::Cyan`)
        .bg(Color::Gray)
        // TO MODIFY TEXT EFFECT: Remove this line if you don't want it bold, 
        // or change `Modifier::BOLD` to `Modifier::ITALIC` or `Modifier::UNDERLINED`.
        .add_modifier(Modifier::BOLD)
}

/// Defines the style for muted, secondary, or disabled text.
pub fn dim() -> Style {
    // TO MODIFY: Change `Color::Gray` to a darker or lighter shade depending on your terminal background.
    Style::default().fg(Color::Gray)
}

/// Defines the style for the status bar (usually at the bottom of the screen).
pub fn status() -> Style {
    Style::default()
        // TO MODIFY TEXT COLOR: Change `Color::White`
        .fg(Color::White)
        // TO MODIFY BACKGROUND: Change `Color::DarkGray` (e.g., `Color::Blue` for a classic blue status bar)
        .bg(Color::DarkGray)
}
use crossterm::event::{KeyCode, KeyEvent};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyAction {
    MoveDown,
    MoveUp,
    Enter,
    Parent,
    SwitchFocus,
    JumpTop,
    JumpBottom,
    StartSearch,
    StartCommand,
    Escape,
    Backspace,
    Submit,
    Char(char),
    None,
}

pub fn map_key(key: KeyEvent) -> KeyAction {
    match key.code {
        KeyCode::Tab => KeyAction::SwitchFocus,
        KeyCode::Char('j') => KeyAction::MoveDown,
        KeyCode::Char('k') => KeyAction::MoveUp,
        KeyCode::Char('l') => KeyAction::Enter,
        KeyCode::Char('h') => KeyAction::Parent,
        KeyCode::Char('G') => KeyAction::JumpBottom,
        KeyCode::Char('/') => KeyAction::StartSearch,
        KeyCode::Char(':') => KeyAction::StartCommand,
        KeyCode::Esc => KeyAction::Escape,
        KeyCode::Backspace => KeyAction::Backspace,
        KeyCode::Enter => KeyAction::Submit,
        KeyCode::Char(ch) => KeyAction::Char(ch),
        _ => KeyAction::None,
    }
}

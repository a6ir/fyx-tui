use crossterm::event::Event as CrosstermEvent;

#[derive(Debug)]
pub enum AppEvent {
    Input(CrosstermEvent),
    Tick,
}

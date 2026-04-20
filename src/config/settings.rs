#[derive(Debug, Clone)]
pub struct Settings {
    pub show_hidden: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self { show_hidden: false }
    }
}

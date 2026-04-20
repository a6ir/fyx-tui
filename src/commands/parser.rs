#[derive(Debug, Clone)]
pub enum ParsedCommand {
    Quit,
    Unknown(String),
}

pub fn parse_command(input: &str) -> ParsedCommand {
    match input.trim() {
        "q" | "quit" => ParsedCommand::Quit,
        other => ParsedCommand::Unknown(other.to_string()),
    }
}

use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum ParsedCommand {
    NoOp,
    Quit,
    Cd(PathBuf),
    Open(PathBuf),
    Invalid(String),
}

pub fn parse_command(input: &str) -> ParsedCommand {
    let trimmed = input.trim().trim_start_matches(':').trim();

    if trimmed.is_empty() {
        return ParsedCommand::NoOp;
    }

    let mut parts = trimmed.split_whitespace();
    let Some(command) = parts.next() else {
        return ParsedCommand::NoOp;
    };

    let args = trimmed[command.len()..].trim();

    match command {
        "q" | "quit" => ParsedCommand::Quit,
        "cd" => {
            if args.is_empty() {
                ParsedCommand::Invalid(String::from("cd requires a path"))
            } else {
                ParsedCommand::Cd(PathBuf::from(args))
            }
        }
        "open" => {
            if args.is_empty() {
                ParsedCommand::Invalid(String::from("open requires a path"))
            } else {
                ParsedCommand::Open(PathBuf::from(args))
            }
        }
        other => ParsedCommand::Invalid(format!("unknown command: {}", other)),
    }
}

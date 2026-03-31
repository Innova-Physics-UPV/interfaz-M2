#[derive(Debug, Clone, Copy)]
pub enum Command {
    Start,
    Stop,
    Status,
}

pub fn handle(cmd: Command) -> String {
    match cmd {
        Command::Start => "OK: start (mock)".to_string(),
        Command::Stop => "OK: stop (mock)".to_string(),
        Command::Status => "OK: status (mock)".to_string(),
    }
}

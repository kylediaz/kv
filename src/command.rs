pub enum Command {
    Ping,
    Echo,
    Command,
    Config,
    Get,
    Set,
    Quit,
}

impl Command {
    pub fn from(input: &Vec<String>) -> Option<Command> {
        match input[0].to_uppercase().as_str() {
            "PING" => Some(Command::Ping),
            "ECHO" => Some(Command::Echo),
            "COMMAND" => Some(Command::Command),
            "CONFIG" => Some(Command::Config),
            "GET" => Some(Command::Get),
            "SET" => Some(Command::Set),
            "QUIT" => Some(Command::Quit),
            _ => None,
        }
    }
}

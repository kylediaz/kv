pub enum Command {
    Ping,
    Echo,
    Command,
    Config,
    Get,
    Set,
    MGet,
    MSet,
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
            "MGET" => Some(Command::MGet),
            "MSET" => Some(Command::MSet),
            "QUIT" => Some(Command::Quit),
            _ => None,
        }
    }
}

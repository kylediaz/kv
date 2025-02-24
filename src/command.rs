pub enum Command {
    Ping,
    Echo,
    Command,
    Config,
    Del,
    Get,
    Incr,
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
            "DEL" => Some(Command::Del),
            "GET" => Some(Command::Get),
            "INCR" => Some(Command::Incr),
            "SET" => Some(Command::Set),
            "MGET" => Some(Command::MGet),
            "MSET" => Some(Command::MSet),
            "QUIT" => Some(Command::Quit),
            _ => None,
        }
    }
}

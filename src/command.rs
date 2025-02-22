pub enum Command {
    Ping,
    Echo,
    Get,
    Set,
    Quit,
}

impl Command {
    pub fn from(input: &Vec<String>) -> Option<Command> {
        match input[0].to_uppercase().as_str() {
            "PING" => Some(Command::Ping),
            "ECHO" => Some(Command::Echo),
            "GET" => Some(Command::Get),
            "SET" => Some(Command::Set),
            "QUIT" => Some(Command::Quit),
            _ => None,
        }
    }
}

pub enum Command {
    Ping,
    Echo,
    Command,
    Config,
    Quit,

    // KV
    Del,
    Get,
    Incr,
    Set,
    MGet,
    MSet,

    // K-Array
    LPush,
    LPop,
    RPush,
    RPop,
}

impl Command {
    pub fn from(input: &Vec<String>) -> Option<Command> {
        match input[0].to_uppercase().as_str() {
            "PING" => Some(Command::Ping),
            "ECHO" => Some(Command::Echo),
            "COMMAND" => Some(Command::Command),
            "CONFIG" => Some(Command::Config),
            "QUIT" => Some(Command::Quit),

            // KV
            "DEL" => Some(Command::Del),
            "GET" => Some(Command::Get),
            "INCR" => Some(Command::Incr),
            "SET" => Some(Command::Set),
            "MGET" => Some(Command::MGet),
            "MSET" => Some(Command::MSet),

            // K-Array
            "LPUSH" => Some(Command::LPush),
            "LPOP" => Some(Command::LPop),
            "RPUSH" => Some(Command::RPush),
            "RPOP" => Some(Command::RPop),
            _ => None,
        }
    }
}

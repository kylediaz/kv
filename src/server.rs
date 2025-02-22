use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Mutex};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

use crate::resp::{bytes_to_resp, RESP};
use crate::storage::Storage;

use super::command::Command;

#[derive(Debug, PartialEq)]
pub enum ServerError {
    CommandError,
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ServerError::CommandError => write!(f, "Command error"),
        }
    }
}

pub type ServerResult<T> = Result<T, ServerError>;

pub struct Server {
    config: Mutex<HashMap<String, String>>,
    storage: Mutex<Storage>,
}

impl Server {
    pub fn new(config: HashMap<String, String>, storage: Mutex<Storage>) -> Self {
        Server {
            config: Mutex::new(config),
            storage,
        }
    }

    pub fn get_config_value(&self, key: &str) -> String {
        let default = "".to_string();
        self.config
            .lock()
            .unwrap()
            .get(key)
            .cloned()
            .unwrap_or(default)
    }

    pub fn set_config_value(&self, key: String, value: String) {
        self.config.lock().unwrap().insert(key.to_string(), value);
    }
}

pub async fn start(config: HashMap<String, String>) -> std::io::Result<()> {
    let default_port = "6379".to_string();
    let port = config.get("port").unwrap_or(&default_port);

    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await?;
    println!("Server initialized");
    let storage = Mutex::new(Storage::new());

    let server: Arc<Server> = Arc::new(Server::new(config, storage));

    println!("Ready to accept connections");
    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                tokio::spawn(handle_connection(stream, server.clone()));
            }
            Err(e) => {
                println!("error: {}", e);
                continue;
            }
        }
    }
}

async fn handle_connection(mut stream: TcpStream, server: Arc<Server>) {
    let mut buffer = [0; 1024];
    loop {
        match stream.read(&mut buffer).await {
            Ok(size) => {
                let mut index: usize = 0;
                let request: RESP = match bytes_to_resp(&buffer[..size].to_vec(), &mut index) {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!("error parsing request: {}", e);
                        return;
                    }
                };
                let response: RESP = match process_request(request, server.clone()) {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!("error processing request: {}", e);
                        return;
                    }
                };
                if let Err(e) = stream.write_all(&response.to_string().as_bytes()).await {
                    eprintln!("error writing response: {}", e);
                    return;
                }
            }
            Ok(_) => {
                println!("connection closed");
                break;
            }
            Err(e) => {
                println!("error: {}", e);
                break;
            }
        }
    }
}

pub fn process_request(request: RESP, server: Arc<Server>) -> ServerResult<RESP> {
    let elements = match request {
        RESP::Array(v) => v,
        _ => {
            return Err(ServerError::CommandError);
        }
    };
    let mut command = Vec::new();
    for elem in elements.iter() {
        match elem {
            RESP::BulkString(s) => command.push(s.clone()),
            _ => {
                return Err(ServerError::CommandError);
            }
        }
    }

    let command_type = Command::from(&command);

    if command_type.is_none() {
        return Err(ServerError::CommandError);
    }
    let command_type = command_type.unwrap();

    return match command_type {
        Command::Ping => {
            if command.len() == 2 {
                Ok(RESP::SimpleString(command[1].to_string()))
            } else {
                Ok(RESP::SimpleString("PONG".to_string()))
            }
        }
        Command::Echo => {
            if command.len() == 2 {
                Ok(RESP::BulkString(command[1].to_string()))
            } else {
                Err(ServerError::CommandError)
            }
        }
        Command::Command => {
            if command[1].eq_ignore_ascii_case("DOCS") {
                Ok(RESP::Array(Vec::new()))
            } else {
                Err(ServerError::CommandError)
            }
        }
        Command::Config => {
            if command[1].eq_ignore_ascii_case("GET") {
                if command.len() == 3 {
                    let key = command[2].to_string();
                    let value = server.get_config_value(&key);
                    Ok(RESP::SimpleString(value))
                } else {
                    Err(ServerError::CommandError)
                }
            } else if command[1].eq_ignore_ascii_case("SET") {
                if command.len() == 4 {
                    let key = command[2].to_string();
                    let value = command[3].to_string();
                    server.set_config_value(key, value);
                    Ok(RESP::SimpleString(command[3].to_string()))
                } else {
                    Err(ServerError::CommandError)
                }
            } else {
                Err(ServerError::CommandError)
            }
        }
        Command::Quit => Ok(RESP::SimpleString("OK".to_string())),
        _ => {
            // Execute command on server
            let result = server.storage.lock().unwrap().process_command(&command);
            return match result {
                Ok(resp) => Ok(resp),
                Err(_) => Err(ServerError::CommandError),
            };
        }
    };
}

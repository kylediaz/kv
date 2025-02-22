use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Mutex};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

use crate::resp::{bytes_to_resp, RESP};
use crate::storage::Storage;

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

pub async fn start(config: HashMap<String, String>) -> std::io::Result<()> {
    let default_port = "6379".to_string();
    let port = config.get("port").unwrap_or(&default_port);

    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await?;
    println!("Server initialized");
    let storage = Arc::new(Mutex::new(Storage::new()));
    loop {
        println!("Ready to accept connections");
        match listener.accept().await {
            Ok((stream, _)) => {
                tokio::spawn(handle_connection(stream, storage.clone()));
            }
            Err(e) => {
                println!("error: {}", e);
                continue;
            }
        }
    }
}

async fn handle_connection(mut stream: TcpStream, storage: Arc<Mutex<Storage>>) {
    let mut buffer = [0; 1024];
    loop {
        match stream.read(&mut buffer).await {
            Ok(size) => {
                let mut index: usize = 0;
                let request = match bytes_to_resp(&buffer[..size].to_vec(), &mut index) {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!("error parsing request: {}", e);
                        return;
                    }
                };
                let response: RESP = match process_request(request, storage.clone()) {
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

pub fn process_request(request: RESP, storage: Arc<Mutex<Storage>>) -> ServerResult<RESP> {
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

    let result = storage.lock().unwrap().process_command(&command);
    return match result {
        Ok(resp) => Ok(resp),
        Err(_) => Err(ServerError::CommandError),
    };
}

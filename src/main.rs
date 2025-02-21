use server::process_request;
use std::collections::HashMap;

use std::env;
use std::sync::Arc;
use std::sync::Mutex;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

mod config;
mod resp;
mod server;
mod storage;

use config::{get_config_from_cli_args, load_config_from_file, load_config_from_stdin};
use resp::{bytes_to_resp, RESP};
use storage::Storage;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("oO0OoO0OoO0Oo Not Redis is starting oO0OoO0OoO0Oo");
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    println!(
        "Not Redis version={}, bits=64, commit=00000000, modified=0, pid={}, just started",
        VERSION,
        std::process::id()
    );

    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    let config_path: String = if args.len() > 0 && !args[0].starts_with("-") {
        args.remove(0)
    } else {
        println!("Warning: no config file specified, using the default config. In order to specify a config file use redis-server /path/to/redis.conf");
        "redis.conf".to_string()
    };

    let use_stdin: bool = if args.len() > 0 && args.last().unwrap().eq(&"-".to_string()) {
        args.pop();
        true
    } else {
        false
    };

    let mut config = HashMap::new();

    // Precedence: CLI args > Stdin > Config file
    load_config_from_file(config_path, &mut config);
    if use_stdin {
        load_config_from_stdin(&mut config);
    }
    get_config_from_cli_args(args, &mut config);

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
                println!("{:}", String::from_utf8_lossy(&buffer[..size]));
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

use std::collections::HashMap;

use std::env;

mod command;
mod config;
mod resp;
mod server;
mod storage;

use config::{get_config_from_cli_args, load_config_from_file, load_config_from_stdin};

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

    return server::start(config).await;
}

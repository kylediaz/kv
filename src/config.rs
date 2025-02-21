use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Lines, Read};

pub fn load_config_from_file(config_path: String, config: &mut HashMap<String, String>) {
    let file = File::open(config_path).expect("unable to open file");
    let reader = BufReader::new(file);
    let lines = reader.lines();
    load_config(lines, config);
}

pub fn load_config_from_stdin(config: &mut HashMap<String, String>) {
    let lines = io::stdin().lines();
    load_config(lines, config);
}

fn load_config<T>(lines: Lines<T>, config: &mut HashMap<String, String>)
where
    T: BufRead + Sized + Read,
{
    for line in lines {
        let line = line.expect("Unable to read line");
        // Skip comments and empty lines
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }
        if let Some((key, value)) = line.split_once(' ') {
            let key = key.trim().to_string();
            let mut value = value.trim().to_string();
            if value.eq("\"\"") {
                value = String::new();
            }
            config.insert(key, value);
        }
    }
}

pub fn get_config_from_cli_args(params: Vec<String>, config: &mut HashMap<String, String>) {
    let mut key: Option<String> = None;
    for arg in params.iter() {
        match arg.strip_prefix("--") {
            Some(k) => {
                key = Some(k.to_string());
            }
            None => match key {
                Some(k) => {
                    config.insert(k, arg.to_string());
                    key = None;
                }
                None => {
                    eprintln!("invalid argument: {}", arg);
                }
            },
        }
    }
}

use std::collections::HashMap;

mod result;

use super::storage::result::{StorageError, StorageResult};
use crate::resp::RESP;

#[derive(Debug, PartialEq, Clone)]
pub enum StorageValue {
    String(String),
}

impl From<StorageValue> for RESP {
    fn from(value: StorageValue) -> RESP {
        match value {
            StorageValue::String(s) => RESP::BulkString(s),
        }
    }
}

pub struct Storage {
    store: HashMap<String, StorageValue>,
}

impl Storage {
    pub fn new() -> Self {
        let store: HashMap<String, StorageValue> = HashMap::new();
        Self { store: store }
    }

    pub fn process_command(&mut self, command: &Vec<String>) -> StorageResult<RESP> {
        match command[0].to_lowercase().as_str() {
            "get" => self.command_get(&command),
            "mget" => self.command_mget(&command),
            "set" => self.command_set(&command),
            "mset" => self.command_mset(&command),
            _ => Err(StorageError::CommandNotAvailable(command[0].clone())),
        }
    }

    fn command_set(&mut self, command: &Vec<String>) -> StorageResult<RESP> {
        if command.len() != 3 {
            return Err(StorageError::CommandSyntaxError(command.join(" ")));
        }
        let _ = self.set(command[1].clone(), command[2].clone());
        Ok(RESP::SimpleString(String::from("OK")))
    }

    fn set(&mut self, key: String, value: String) -> StorageResult<String> {
        self.store.insert(key, StorageValue::String(value));
        Ok(String::from("OK"))
    }

    fn command_mset(&mut self, command: &Vec<String>) -> StorageResult<RESP> {
        if command.len() % 2 != 1 {
            return Err(StorageError::CommandSyntaxError(command.join(" ")));
        }
        for i in (1..command.len()).step_by(2) {
            let _ = self.set(command[i].clone(), command[i + 1].clone());
        }
        Ok(RESP::SimpleString(String::from("OK")))
    }

    fn command_get(&mut self, command: &Vec<String>) -> StorageResult<RESP> {
        if command.len() != 2 {
            return Err(StorageError::CommandSyntaxError(command.join(" ")));
        }
        let output = self.get(command[1].clone());
        match output {
            Ok(Some(v)) => Ok(RESP::BulkString(v)),
            Ok(None) => Ok(RESP::Null),
            Err(_) => Err(StorageError::CommandInternalError(command.join(" "))),
        }
    }

    fn get(&self, key: String) -> StorageResult<Option<String>> {
        match self.store.get(&key) {
            Some(StorageValue::String(v)) => return Ok(Some(v.clone())),
            None => return Ok(None),
        }
    }

    fn command_mget(&mut self, command: &Vec<String>) -> StorageResult<RESP> {
        if command.len() < 2 {
            return Err(StorageError::CommandSyntaxError(command.join(" ")));
        }
        let mut values = Vec::new();
        for i in 1..command.len() {
            let key = command.get(i);
            if let None = key {
                values.push(RESP::Null);
            } else {
                let key = key.unwrap();
                let value: RESP = match self.store.get(key) {
                    Some(v) => v.clone().into(),
                    None => RESP::Null,
                };
                values.push(value);
            }
        }
        Ok(RESP::Array(values))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_new() {
        let storage: Storage = Storage::new();
        assert_eq!(storage.store.len(), 0);
    }

    #[test]
    fn test_process_command_set() {
        let mut storage: Storage = Storage::new();
        let command = vec![
            String::from("set"),
            String::from("key"),
            String::from("value"),
        ];
        let output = storage.process_command(&command).unwrap();
        assert_eq!(output, RESP::SimpleString(String::from("OK")));
        assert_eq!(storage.store.len(), 1);
    }

    #[test]
    fn test_process_command_get() {
        let mut storage: Storage = Storage::new();
        storage.store.insert(
            String::from("akey"),
            StorageValue::String(String::from("avalue")),
        );
        let command = vec![String::from("get"), String::from("akey")];
        let output = storage.process_command(&command).unwrap();
        assert_eq!(output, RESP::BulkString(String::from("avalue")));
        assert_eq!(storage.store.len(), 1);
    }

    #[test]
    fn test_process_command_set_and_get() {
        let mut storage: Storage = Storage::new();
        let command = vec![
            String::from("set"),
            String::from("key"),
            String::from("value"),
        ];
        let output = storage.process_command(&command).unwrap();
        assert_eq!(output, RESP::SimpleString(String::from("OK")));
        assert_eq!(storage.store.len(), 1);

        let command = vec![String::from("get"), String::from("key")];
        let output = storage.process_command(&command).unwrap();
        assert_eq!(output, RESP::BulkString(String::from("value")));
        assert_eq!(storage.store.len(), 1);
    }

    #[test]
    fn test_process_command_mget() {
        let mut storage: Storage = Storage::new();
        storage.store.insert(
            String::from("akey1"),
            StorageValue::String(String::from("avalue1")),
        );
        storage.store.insert(
            String::from("akey2"),
            StorageValue::String(String::from("avalue2")),
        );

        let command = vec![
            String::from("mget"),
            String::from("akey1"),
            String::from("akey2"),
        ];
        let output = storage.process_command(&command).unwrap();
        assert_eq!(
            output,
            RESP::Array(vec![
                RESP::BulkString(String::from("avalue1")),
                RESP::BulkString(String::from("avalue2"))
            ])
        );
        assert_eq!(storage.store.len(), 2);
    }

    #[test]
    fn test_process_command_mset() {
        let mut storage: Storage = Storage::new();

        let command = vec![
            String::from("mset"),
            String::from("akey1"),
            String::from("avalue1"),
            String::from("akey2"),
            String::from("avalue2"),
        ];
        let output = storage.process_command(&command).unwrap();
        assert_eq!(output, RESP::SimpleString(String::from("OK")));
    }
}

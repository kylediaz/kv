use std::collections::HashMap;

mod result;

use super::storage::result::{StorageError, StorageResult};
use crate::quicklist::{Dequeue, Quicklist};
use crate::resp::RESP;

#[derive(Debug, PartialEq, Clone)]
pub enum PrimitiveStorageValue {
    String(String),
    Integer(i64),
}

pub enum StorageValue {
    Primitive(PrimitiveStorageValue),
    Quicklist(Quicklist<PrimitiveStorageValue>),
}

impl From<PrimitiveStorageValue> for RESP {
    fn from(value: PrimitiveStorageValue) -> RESP {
        match value {
            PrimitiveStorageValue::String(s) => RESP::BulkString(s),
            PrimitiveStorageValue::Integer(i) => RESP::Integer(i),
        }
    }
}

impl From<String> for StorageValue {
    fn from(value: String) -> Self {
        StorageValue::Primitive(PrimitiveStorageValue::String(value))
    }
}

impl From<i64> for StorageValue {
    fn from(value: i64) -> Self {
        StorageValue::Primitive(PrimitiveStorageValue::Integer(value))
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
            "del" => self.command_del(&command),
            "incr" => self.command_incr(&command),
            "lpush" => self.command_lpush(&command),
            "lpop" => self.command_lpop(&command),
            "rpush" => self.command_rpush(&command),
            "rpop" => self.command_rpop(&command),
            _ => Err(StorageError::CommandNotAvailable(command[0].clone())),
        }
    }

    fn command_set(&mut self, command: &Vec<String>) -> StorageResult<RESP> {
        if command.len() != 3 {
            let command = command.join(" ");
            return Err(StorageError::CommandSyntaxError(
                command,
                "Expected 3 arguments".to_string(),
            ));
        }
        let _ = self.set(command[1].clone(), command[2].clone());
        Ok(RESP::SimpleString(String::from("OK")))
    }

    fn set(&mut self, key: String, value: String) -> StorageResult<String> {
        self.store.insert(
            key,
            StorageValue::Primitive(PrimitiveStorageValue::String(value)),
        );
        Ok(String::from("OK"))
    }

    fn command_mset(&mut self, command: &Vec<String>) -> StorageResult<RESP> {
        if command.len() == 1 {
            return Err(StorageError::CommandSyntaxError(
                command.join(" "),
                "Expected arguments".to_string(),
            ));
        }
        if command.len() % 2 != 1 {
            let command = command.join(" ");
            return Err(StorageError::CommandSyntaxError(
                command,
                "Expected an even number of arguments".to_string(),
            ));
        }
        for i in (1..command.len()).step_by(2) {
            let _ = self.set(command[i].clone(), command[i + 1].clone());
        }
        Ok(RESP::SimpleString(String::from("OK")))
    }

    fn command_get(&mut self, command: &Vec<String>) -> StorageResult<RESP> {
        if command.len() != 2 {
            return Err(StorageError::CommandSyntaxError(
                command.join(" "),
                "Expected an argument".to_string(),
            ));
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
            Some(StorageValue::Primitive(p)) => match p {
                PrimitiveStorageValue::String(v) => return Ok(Some(v.clone())),
                PrimitiveStorageValue::Integer(v) => return Ok(Some(v.to_string())),
            },
            Some(_) => return Err(StorageError::WrongType),
            None => return Ok(None),
        }
    }

    fn command_mget(&mut self, command: &Vec<String>) -> StorageResult<RESP> {
        if command.len() < 2 {
            return Err(StorageError::CommandSyntaxError(
                command.join(" "),
                "Expected at least one argument".to_string(),
            ));
        }
        let mut values = Vec::new();
        for i in 1..command.len() {
            let key = command.get(i);
            if let None = key {
                values.push(RESP::Null);
            } else {
                let key = key.unwrap();
                let value: RESP = match self.get(key.to_string()) {
                    Ok(None) => RESP::Null,
                    Ok(Some(v)) => RESP::BulkString(v),
                    Err(e) => return Err(e),
                };
                values.push(value);
            }
        }
        Ok(RESP::Array(values))
    }

    fn command_del(&mut self, command: &Vec<String>) -> StorageResult<RESP> {
        if command.len() < 2 {
            let command = command.join(" ");
            return Err(StorageError::CommandSyntaxError(
                command,
                "Expected at least one argument".to_string(),
            ));
        }
        let mut count = 0;
        for i in 1..command.len() {
            let key = command.get(i).unwrap();
            match self.store.remove(key) {
                Some(_) => {
                    count += 1;
                }
                None => continue,
            };
        }
        Ok(RESP::Integer(count))
    }

    fn command_incr(&mut self, command: &Vec<String>) -> StorageResult<RESP> {
        if command.len() != 2 {
            return Err(StorageError::CommandSyntaxError(
                command.join(" "),
                "Expected exactly one argument".to_string(),
            ));
        }
        let key = command.get(1).unwrap();
        match self.store.get_mut(key) {
            Some(StorageValue::Primitive(v)) => match v {
                PrimitiveStorageValue::String(value) => match value.parse::<i64>() {
                    Ok(parsed_value) => {
                        let new_value = parsed_value + 1;
                        *value = new_value.to_string();
                        Ok(RESP::Integer(new_value))
                    }
                    Err(_) => Err(StorageError::ValueNotInteger(value.clone())),
                },
                PrimitiveStorageValue::Integer(value) => {
                    *value += 1;
                    Ok(RESP::Integer(*value))
                }
            },
            Some(_) => Err(StorageError::WrongType),
            None => {
                self.store.insert(
                    key.clone(),
                    StorageValue::Primitive(PrimitiveStorageValue::Integer(1)),
                );
                Ok(RESP::Integer(1))
            }
        }
    }

    fn command_lpush(&mut self, command: &Vec<String>) -> StorageResult<RESP> {
        if command.len() != 3 {
            return Err(StorageError::CommandSyntaxError(
                command.join(" "),
                "Expected LPUSH [key] [value]".to_string(),
            ));
        }
        let key = command.get(1).unwrap();
        let value = command.get(2).unwrap();
        let storage_value = PrimitiveStorageValue::String(value.to_string());
        match self.store.get_mut(key) {
            Some(StorageValue::Quicklist(l)) => {
                l.lpush(storage_value);
                Ok(RESP::SimpleString(String::from("OK")))
            }
            None => {
                let mut l = Quicklist::new();
                l.lpush(storage_value);
                self.store
                    .insert(key.to_string(), StorageValue::Quicklist(l));
                Ok(RESP::SimpleString(String::from("OK")))
            }
            Some(_) => Err(StorageError::WrongType),
        }
    }

    fn command_lpop(&mut self, command: &Vec<String>) -> StorageResult<RESP> {
        if command.len() != 2 {
            return Err(StorageError::CommandSyntaxError(
                command.join(" "),
                "Expected LPOP [key]".to_string(),
            ));
        }
        let key = command.get(1).unwrap();
        match self.store.get_mut(key) {
            Some(StorageValue::Quicklist(l)) => {
                let value = l.lpop();
                Ok(value.into())
            }
            None => Err(StorageError::KeyNotFound(key.to_string())),
            Some(_) => Err(StorageError::WrongType),
        }
    }

    fn command_rpush(&mut self, command: &Vec<String>) -> StorageResult<RESP> {
        if command.len() != 3 {
            return Err(StorageError::CommandSyntaxError(
                command.join(" "),
                "Expected RPUSH [key] [value]".to_string(),
            ));
        }
        let key = command.get(1).unwrap();
        let value = command.get(2).unwrap();
        let storage_value = PrimitiveStorageValue::String(value.to_string());
        match self.store.get_mut(key) {
            Some(StorageValue::Quicklist(l)) => {
                l.rpush(storage_value);
                Ok(RESP::SimpleString(String::from("OK")))
            }
            None => {
                let mut l = Quicklist::new();
                l.rpush(storage_value);
                self.store
                    .insert(key.to_string(), StorageValue::Quicklist(l));
                Ok(RESP::SimpleString(String::from("OK")))
            }
            Some(_) => Err(StorageError::WrongType),
        }
    }

    fn command_rpop(&mut self, command: &Vec<String>) -> StorageResult<RESP> {
        if command.len() != 2 {
            return Err(StorageError::CommandSyntaxError(
                command.join(" "),
                "Expected RPOP [key]".to_string(),
            ));
        }
        let key = command.get(1).unwrap();
        match self.store.get_mut(key) {
            Some(StorageValue::Quicklist(l)) => {
                let value = l.rpop();
                Ok(value.into())
            }
            None => Err(StorageError::KeyNotFound(key.to_string())),
            Some(_) => Err(StorageError::WrongType),
        }
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
        storage
            .store
            .insert(String::from("akey"), String::from("avalue").into());
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
        storage
            .store
            .insert(String::from("akey1"), String::from("avalue1").into());
        storage
            .store
            .insert(String::from("akey2"), String::from("avalue2").into());

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

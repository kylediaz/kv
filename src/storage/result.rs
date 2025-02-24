use std::fmt;

#[derive(Debug)]
pub enum StorageError {
    IncorrectRequest,
    CommandInternalError(String),
    CommandSyntaxError(String, String),
    CommandNotAvailable(String),
    ValueNotInteger(String),
    KeyNotFound(String),
}

impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StorageError::IncorrectRequest => write!(f, "Incorrect request"),
            StorageError::CommandInternalError(command) => {
                write!(f, "Command internal error: {}", command)
            }
            StorageError::CommandSyntaxError(command, message) => {
                write!(f, "Command syntax error: {} - {}", command, message)
            }
            StorageError::CommandNotAvailable(command) => {
                write!(f, "Command not available: {}", command)
            }
            StorageError::ValueNotInteger(value) => {
                write!(f, "Value not an integer: {}", value)
            }
            StorageError::KeyNotFound(key) => {
                write!(f, "Key not found: {}", key)
            }
        }
    }
}

pub type StorageResult<T> = Result<T, StorageError>;

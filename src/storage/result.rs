use std::fmt;

#[derive(Debug)]
pub enum StorageError {
    IncorrectRequest,
    CommandInternalError(String),
    CommandSyntaxError(String),
    CommandNotAvailable(String),
}

impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StorageError::IncorrectRequest => write!(f, "Incorrect request"),
            StorageError::CommandInternalError(command) => {
                write!(f, "Command internal error: {}", command)
            }
            StorageError::CommandSyntaxError(command) => {
                write!(f, "Command syntax error: {}", command)
            }
            StorageError::CommandNotAvailable(command) => {
                write!(f, "Command not available: {}", command)
            }
        }
    }
}

pub type StorageResult<T> = Result<T, StorageError>;

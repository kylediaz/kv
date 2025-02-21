use std::fmt;
use std::sync::{Arc, Mutex};

use crate::storage::Storage;
use crate::RESP;

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

use std::fmt;

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

pub fn process_request(request: RESP) -> ServerResult<RESP> {
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

    match command[0].to_lowercase().as_str() {
        "ping" => Ok(RESP::SimpleString("PONG".to_string())),
        "echo" => Ok(RESP::BulkString(command[1].clone())),
        _ => Err(ServerError::CommandError),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_request_ping() {
        let request = RESP::Array(vec![RESP::BulkString("PING".to_string())]);
        let response = process_request(request);
        assert_eq!(response, Ok(RESP::SimpleString("PONG".to_string())));
    }

    #[test]
    fn test_process_request_echo() {
        let request = RESP::Array(vec![
            RESP::BulkString("ECHO".to_string()),
            RESP::BulkString("Hello".to_string()),
        ]);
        let response = process_request(request);
        assert_eq!(response, Ok(RESP::BulkString("Hello".to_string())));
    }
}

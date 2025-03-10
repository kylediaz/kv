use std::fmt;
use std::num;
use std::string;

pub type RESPLength = i32;

#[derive(Debug, PartialEq)]
pub enum RESPError {
    FromUtf8,
    IncorrectLength(RESPLength),
    OutOfBounds(usize),
    ParseInt,
    Unknown,
    WrongType,
}

impl fmt::Display for RESPError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RESPError::FromUtf8 => write!(f, "Cannot convert from UTF-8"),
            RESPError::IncorrectLength(length) => write!(f, "Incorrect legth {}", length),
            RESPError::OutOfBounds(index) => write!(f, "Index out of bounds: {}", index),
            RESPError::Unknown => write!(f, "Unknown format for RESP string"),
            RESPError::ParseInt => write!(f, "Cannot parse string into integer"),
            RESPError::WrongType => write!(f, "Wrong prefix for RESP type"),
        }
    }
}

impl From<string::FromUtf8Error> for RESPError {
    fn from(_err: string::FromUtf8Error) -> Self {
        Self::FromUtf8
    }
}

impl From<num::ParseIntError> for RESPError {
    fn from(_err: num::ParseIntError) -> Self {
        Self::ParseInt
    }
}

pub type RESPResult<T> = Result<T, RESPError>;

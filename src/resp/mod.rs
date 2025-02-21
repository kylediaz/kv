mod result;
mod util;

use std::fmt::{self, Display};

use crate::resp::result::{RESPError, RESPLength, RESPResult};
use crate::resp::util::*;

#[derive(Debug, PartialEq)]
pub enum RESP {
    Array(Vec<RESP>),
    BulkString(String),
    Null,
    SimpleString(String),
}

impl Display for RESP {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Array(array) => {
                write!(f, "*{}\r\n", array.len())?;
                for item in array {
                    write!(f, "{}", item)?;
                }
                Ok(())
            }
            Self::BulkString(s) => write!(f, "${}\r\n{}\r\n", s.len(), s),
            Self::Null => write!(f, "$-1\r\n"),
            Self::SimpleString(s) => write!(f, "+{}\r\n", s),
        }
    }
}

pub fn bytes_to_resp(buffer: &[u8], index: &mut usize) -> RESPResult<RESP> {
    match parser_router(buffer, index) {
        Some(parse_func) => {
            let result: RESP = parse_func(buffer, index)?;
            Ok(result)
        }
        None => Err(RESPError::Unknown),
    }
}

fn parser_router(
    buffer: &[u8],
    index: &mut usize,
) -> Option<fn(&[u8], &mut usize) -> RESPResult<RESP>> {
    match buffer[*index] {
        b'*' => Some(parse_array),
        b'$' => Some(parse_bulk_string),
        b'+' => Some(parse_simple_string),
        _ => None,
    }
}

fn parse_array(buffer: &[u8], index: &mut usize) -> RESPResult<RESP> {
    resp_remove_type('*', buffer, index)?;
    let length = resp_extract_length(buffer, index)?;
    if length < 0 {
        return Err(RESPError::IncorrectLength(length));
    }
    let mut data = Vec::new();
    for _ in 0..length {
        if *index >= buffer.len() {
            return Err(RESPError::OutOfBounds(*index));
        }
        match parser_router(buffer, index) {
            Some(parse_func) => {
                let array_element: RESP = parse_func(buffer, index)?;
                data.push(array_element);
            }
            None => return Err(RESPError::Unknown),
        }
    }
    Ok(RESP::Array(data))
}

fn parse_bulk_string(buffer: &[u8], index: &mut usize) -> RESPResult<RESP> {
    resp_remove_type('$', buffer, index)?;
    let length = resp_extract_length(buffer, index)?;
    if length == -1 {
        return Ok(RESP::Null);
    }
    if length < -1 {
        return Err(RESPError::IncorrectLength(length));
    }
    let bytes = binary_extract_bytes(buffer, index, length as usize)?;
    let data: String = String::from_utf8(bytes)?;
    // Increment the index to skip the \r\n
    *index += 2;
    Ok(RESP::BulkString(data))
}

fn parse_simple_string(buffer: &[u8], index: &mut usize) -> RESPResult<RESP> {
    resp_remove_type('+', buffer, index)?;
    let line: String = binary_extract_line_as_string(buffer, index)?;
    Ok(RESP::SimpleString(line))
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! parse_test {
        ($name:ident, $buffer:expr, $expected:expr, $expected_index:expr) => {
            #[test]
            fn $name() {
                let buffer = $buffer.as_bytes();
                let mut index: usize = 0;
                let result = bytes_to_resp(buffer, &mut index).unwrap();
                assert_eq!(result, $expected);
                assert_eq!(index, $expected_index);
                assert_eq!(result.to_string(), $buffer);
            }
        };
    }

    macro_rules! parse_test_expect_error {
        ($name:ident, $buffer:expr, $expected_error:expr, $expected_index:expr) => {
            #[test]
            fn $name() {
                let buffer = $buffer.as_bytes();
                let mut index: usize = 0;
                let error = bytes_to_resp(buffer, &mut index).unwrap_err();
                assert_eq!(error, $expected_error);
                assert_eq!(index, $expected_index);
            }
        };
    }

    parse_test!(
        test_parse_array_simple,
        "*1\r\n+hello\r\n",
        RESP::Array(vec![RESP::SimpleString(String::from("hello"))]),
        12
    );

    parse_test!(
        test_parse_array_bulk_string,
        "*1\r\n$5\r\nhello\r\n",
        RESP::Array(vec![RESP::BulkString(String::from("hello"))]),
        15
    );

    parse_test!(
        test_parse_array_multiple,
        "*2\r\n+hello\r\n+world\r\n",
        RESP::Array(vec![
            RESP::SimpleString(String::from("hello")),
            RESP::SimpleString(String::from("world"))
        ]),
        20
    );

    parse_test!(
        test_parse_array_multiple_bulk,
        "*2\r\n+hello\r\n$5\r\nworld\r\n",
        RESP::Array(vec![
            RESP::SimpleString(String::from("hello")),
            RESP::BulkString(String::from("world"))
        ]),
        23
    );

    parse_test!(
        test_parse_array_multiple_mixed,
        "*2\r\n+hello\r\n$5\r\nworld\r\n",
        RESP::Array(vec![
            RESP::SimpleString(String::from("hello")),
            RESP::BulkString(String::from("world"))
        ]),
        23
    );

    parse_test!(test_parse_array_empty, "*0\r\n", RESP::Array(vec![]), 4);

    parse_test_expect_error!(
        test_parse_array_out_of_bounds,
        "*2\r\n+hello\r\n",
        RESPError::OutOfBounds(12),
        12
    );

    parse_test!(
        test_parse_simple_string,
        "+OK\r\n",
        RESP::SimpleString(String::from("OK")),
        5
    );

    parse_test_expect_error!(test_bytes_to_resp_unknown, "?OK\r\n", RESPError::Unknown, 0);
}

mod result;

use std::fmt::{self, Display};

use crate::resp::result::{RESPError, RESPLength, RESPResult};

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
            Self::BulkString(s) => write!(f, "${}\r\n{}", s.len(), s),
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

// Parse array

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

fn resp_remove_type(value: char, buffer: &[u8], index: &mut usize) -> RESPResult<()> {
    if buffer[*index] != value as u8 {
        return Err(RESPError::WrongType);
    }
    *index += 1;
    Ok(())
}

fn resp_extract_length(buffer: &[u8], index: &mut usize) -> RESPResult<RESPLength> {
    let line: String = binary_extract_line_as_string(buffer, index)?;
    let length: RESPLength = line.parse()?;
    Ok(length)
}

// Parse bulk string

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

fn binary_extract_bytes(buffer: &[u8], index: &mut usize, length: usize) -> RESPResult<Vec<u8>> {
    let mut output = Vec::new();
    if *index + length > buffer.len() {
        return Err(RESPError::OutOfBounds(*index + buffer.len()));
    }
    output.extend_from_slice(&buffer[*index..*index + length]);
    *index += length;
    Ok(output)
}

// Parse simple string

fn parse_simple_string(buffer: &[u8], index: &mut usize) -> RESPResult<RESP> {
    resp_remove_type('+', buffer, index)?;
    let line: String = binary_extract_line_as_string(buffer, index)?;
    Ok(RESP::SimpleString(line))
}

fn binary_extract_line_as_string(buffer: &[u8], index: &mut usize) -> RESPResult<String> {
    let line = binary_extract_line(buffer, index)?;
    Ok(String::from_utf8(line)?)
}

fn binary_extract_line(buffer: &[u8], index: &mut usize) -> RESPResult<Vec<u8>> {
    if *index >= buffer.len() {
        return Err(RESPError::OutOfBounds(*index));
    }
    if buffer.len() - *index - 1 < 2 {
        return Err(RESPError::OutOfBounds(buffer.len()));
    }

    for i in *index..(buffer.len() - 1) {
        if buffer[i] == b'\r' && buffer[i + 1] == b'\n' {
            let output = buffer[*index..i].to_vec();
            *index = i + 2;
            return Ok(output);
        }
    }

    Err(RESPError::OutOfBounds(buffer.len()))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_array_simple() {
        let buffer = "*1\r\n+hello\r\n".as_bytes();
        let mut index: usize = 0;
        let result = parse_array(buffer, &mut index).unwrap();
        assert_eq!(
            result,
            RESP::Array(vec![RESP::SimpleString(String::from("hello"))])
        );
        assert_eq!(index, 12);
    }

    #[test]
    fn test_parse_array_bulk_string() {
        let buffer = "*1\r\n$5\r\nhello\r\n".as_bytes();
        let mut index: usize = 0;
        let result = parse_array(buffer, &mut index).unwrap();
        assert_eq!(
            result,
            RESP::Array(vec![RESP::BulkString(String::from("hello"))])
        );
        assert_eq!(index, 15);
    }

    #[test]
    fn test_parse_array_multiple() {
        let buffer = "*2\r\n+hello\r\n+world\r\n".as_bytes();
        let mut index: usize = 0;
        let result = parse_array(buffer, &mut index).unwrap();
        assert_eq!(
            result,
            RESP::Array(vec![
                RESP::SimpleString(String::from("hello")),
                RESP::SimpleString(String::from("world"))
            ])
        );
        assert_eq!(index, 20);
    }

    #[test]
    fn test_parse_array_empty() {
        let buffer = "*0\r\n".as_bytes();
        let mut index: usize = 0;
        let result = parse_array(buffer, &mut index).unwrap();
        assert_eq!(result, RESP::Array(vec![]));
        assert_eq!(index, 4);
    }

    #[test]
    fn test_parse_array_out_of_bounds() {
        let buffer = "*2\r\n+hello\r\n".as_bytes();
        let mut index: usize = 0;
        let result: RESPError = parse_array(buffer, &mut index).unwrap_err();
        assert_eq!(result, RESPError::OutOfBounds(12));
        assert_eq!(index, 12);
    }

    #[test]
    fn test_binary_extract_bytes() {
        let buffer = "SOMEBYTES".as_bytes();
        let mut index: usize = 0;
        let output = binary_extract_bytes(buffer, &mut index, 6).unwrap();
        assert_eq!(output, "SOMEBY".as_bytes().to_vec());
        assert_eq!(index, 6);
    }
    #[test]
    fn test_binary_extract_bytes_out_of_bounds() {
        let buffer = "SOMEBYTES".as_bytes();
        let mut index: usize = 0;
        let error = binary_extract_bytes(buffer, &mut index, 10).unwrap_err();
        assert_eq!(error, RESPError::OutOfBounds(9));
        assert_eq!(index, 0);
    }

    #[test]
    fn test_parse_simple_string() {
        let buffer = "+OK\r\n".as_bytes();
        let mut index: usize = 0;
        let result = parse_simple_string(buffer, &mut index).unwrap();

        assert_eq!(result, RESP::SimpleString(String::from("OK")));
        assert_eq!(index, 5);
    }

    #[test]
    fn test_bytes_to_resp_simple_string() {
        let buffer = "+OK\r\n".as_bytes();
        let mut index: usize = 0;
        let output = bytes_to_resp(buffer, &mut index).unwrap();
        assert_eq!(output, RESP::SimpleString(String::from("OK")));
        assert_eq!(index, 5);
    }
    #[test]
    fn test_bytes_to_resp_unknown() {
        let buffer = "?OK\r\n".as_bytes();
        let mut index: usize = 0;
        let error = bytes_to_resp(buffer, &mut index).unwrap_err();
        assert_eq!(error, RESPError::Unknown);
        assert_eq!(index, 0);
    }

    #[test]
    fn test_binary_remove_type() {
        let buffer = "+OK\r\n".as_bytes();
        let mut index: usize = 0;
        resp_remove_type('+', buffer, &mut index).unwrap();
        assert_eq!(index, 1);
    }
    #[test]
    fn test_binary_remove_type_error() {
        let buffer = "*OK\r\n".as_bytes();
        let mut index: usize = 0;
        let error = resp_remove_type('+', buffer, &mut index).unwrap_err();
        assert_eq!(index, 0);
        assert_eq!(error, RESPError::WrongType);
    }

    #[test]
    fn test_binary_extract_line_empty_buffer() {
        let buffer = b"";
        let mut index = 0;
        let result = binary_extract_line(buffer, &mut index);
        match result {
            Err(RESPError::OutOfBounds(0)) => return,
            _ => panic!("Unexpected result"),
        }
    }

    #[test]
    fn test_binary_extract_single_character() {
        let buffer = b"a\r\n";
        let mut index = 0;
        let result = binary_extract_line(buffer, &mut index);
        match result {
            Ok(output) => {
                assert_eq!(output, b"a".to_vec());
                assert_eq!(index, 3);
            }
            _ => panic!("Unexpected result"),
        }
    }

    #[test]
    fn test_binary_extract_line_single_line() {
        let buffer = b"hello\r\n";
        let mut index = 0;
        let result = binary_extract_line(buffer, &mut index);
        match result {
            Ok(output) => {
                assert_eq!(output, b"hello".to_vec());
                assert_eq!(index, 7);
            }
            _ => panic!("Unexpected result"),
        }
    }

    #[test]
    fn test_binary_extract_index_out_of_bounds() {
        let buffer = b"hello\r\n";
        let mut index = 10;
        let result = binary_extract_line(buffer, &mut index);
        match result {
            Err(RESPError::OutOfBounds(10)) => return,
            _ => panic!("Unexpected result"),
        }
    }

    #[test]
    fn test_binary_extract_line_no_separator() {
        let buffer = "OK".as_bytes();
        let mut index: usize = 0;
        match binary_extract_line(buffer, &mut index) {
            Err(RESPError::OutOfBounds(2)) => return,
            _ => panic!("Unexpected result"),
        }
    }

    #[test]
    fn test_binary_extract_line_half_separator() {
        let buffer = "OK\r".as_bytes();
        let mut index: usize = 0;
        match binary_extract_line(buffer, &mut index) {
            Err(RESPError::OutOfBounds(index)) => {
                assert_eq!(index, 3);
            }
            _ => panic!(),
        }
    }

    #[test]
    fn test_binary_extract_line_incorrect_separator() {
        let buffer = "OK\n".as_bytes();
        let mut index: usize = 0;
        match binary_extract_line(buffer, &mut index) {
            Err(RESPError::OutOfBounds(3)) => return,
            _ => panic!(),
        }
    }

    #[test]
    fn test_binary_extract_line() {
        let buffer = "OK\r\n".as_bytes();
        let mut index: usize = 0;
        let output = binary_extract_line(buffer, &mut index).unwrap();
        assert_eq!(output, "OK".as_bytes());
        assert_eq!(index, 4);
    }
}

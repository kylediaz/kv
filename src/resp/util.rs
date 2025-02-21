use super::result::{RESPError, RESPLength, RESPResult};

pub fn binary_extract_bytes(
    buffer: &[u8],
    index: &mut usize,
    length: usize,
) -> RESPResult<Vec<u8>> {
    let mut output = Vec::new();
    if *index + length > buffer.len() {
        return Err(RESPError::OutOfBounds(*index + buffer.len()));
    }
    output.extend_from_slice(&buffer[*index..*index + length]);
    *index += length;
    Ok(output)
}

pub fn resp_remove_type(value: char, buffer: &[u8], index: &mut usize) -> RESPResult<()> {
    if buffer[*index] != value as u8 {
        return Err(RESPError::WrongType);
    }
    *index += 1;
    Ok(())
}

pub fn resp_extract_length(buffer: &[u8], index: &mut usize) -> RESPResult<RESPLength> {
    let line: String = binary_extract_line_as_string(buffer, index)?;
    let length: RESPLength = line.parse()?;
    Ok(length)
}

pub fn binary_extract_line_as_string(buffer: &[u8], index: &mut usize) -> RESPResult<String> {
    let line = binary_extract_line(buffer, index)?;
    Ok(String::from_utf8(line)?)
}

pub fn binary_extract_line(buffer: &[u8], index: &mut usize) -> RESPResult<Vec<u8>> {
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
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if let Some(invalid_char) = string_digits.chars().find(|ch| !ch.is_ascii_digit()) {
        return Err(Error::InvalidDigit(invalid_char));
    }

    match span {
        0 => Ok(1),
        _ => string_digits
            .as_bytes()
            .windows(span)
            .map(product)
            .max()
            .ok_or(Error::SpanTooLong),
    }
}

fn product(window: &[u8]) -> u64 {
    window.iter().map(|&byte| (byte - b'0') as u64).product()
}

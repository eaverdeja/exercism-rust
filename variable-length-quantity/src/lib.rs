#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
}

pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    values
        .iter()
        .flat_map(|&value| {
            let parts = chunk_parts(value);
            process_parts(&parts)
        })
        .collect()
}

fn chunk_parts(value: u32) -> Vec<u8> {
    if value == 0 {
        return vec![0];
    }

    let mut parts = Vec::with_capacity(5);
    let mut temp = value;
    while temp > 0 {
        parts.push((temp & 0x7F) as u8);
        temp >>= 7;
    }

    parts
}

fn process_parts(parts: &[u8]) -> Vec<u8> {
    parts
        .iter()
        .enumerate()
        .rev()
        .map(|(i, &byte)| if i > 0 { byte | 0x80 } else { byte })
        .collect()
}

pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut result = Vec::new();
    let mut i = 0;

    while i < bytes.len() {
        let (value, bytes_read) = decode_single_number(&bytes[i..])?;
        result.push(value);
        i += bytes_read;
    }

    Ok(result)
}

fn decode_single_number(bytes: &[u8]) -> Result<(u32, usize), Error> {
    let mut value: u32 = 0;
    let mut i = 0;

    loop {
        if i >= bytes.len() {
            return Err(Error::IncompleteNumber);
        }

        // Read the next byte, get the least significant 7 bits
        // out of it and make room for them on value
        let byte = bytes[i];
        value = (value << 7) | (byte & 0x7F) as u32;
        i += 1;

        // If the MSB is 0, this is the last byte for this number
        if byte & 0x80 == 0 {
            break;
        }
    }

    Ok((value, i))
}

const ALPHABET_SIZE: i32 = 26;

#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if !is_coprime(a) {
        return Err(AffineCipherError::NotCoprime(a));
    }

    Ok(plaintext
        .to_lowercase()
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric())
        .map(|ch| encode_char(ch, a, b))
        .collect::<Vec<char>>()
        .chunks(5)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join(" "))
}

pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    let mmi = mmi(a)?;

    Ok(ciphertext
        .chars()
        .filter(|ch| !ch.is_whitespace())
        .map(|ch| decode_char(ch, mmi, b))
        .collect())
}

fn encode_char(ch: char, a: i32, b: i32) -> char {
    if ch.is_ascii_digit() {
        ch
    } else {
        let i = from_ascii(ch);
        // E(x) = (ai + b) mod m
        to_ascii((a * i + b).rem_euclid(ALPHABET_SIZE))
    }
}

fn decode_char(ch: char, mmi: i32, b: i32) -> char {
    if ch.is_ascii_digit() {
        ch
    } else {
        let y = from_ascii(ch);
        // D(y) = (a^-1)(y - b) mod m
        to_ascii((mmi * (y - b)).rem_euclid(ALPHABET_SIZE))
    }
}

fn is_coprime(a: i32) -> bool {
    gcd(a, ALPHABET_SIZE) == 1
}

fn gcd(a: i32, m: i32) -> i32 {
    if m == 0 {
        return a;
    }
    gcd(m, a % m)
}

fn mmi(a: i32) -> Result<i32, AffineCipherError> {
    if !is_coprime(a) {
        return Err(AffineCipherError::NotCoprime(a));
    }

    // Simple approach since m is known and small
    for i in 1..ALPHABET_SIZE {
        if (a * i) % ALPHABET_SIZE == 1 {
            return Ok(i);
        }
    }

    unreachable!("`a` is coprime with ALPHABET_SIZE")
}

fn from_ascii(ch: char) -> i32 {
    (ch as u8 - b'a') as i32
}

fn to_ascii(alpha_idx: i32) -> char {
    (alpha_idx as u8 + b'a') as char
}

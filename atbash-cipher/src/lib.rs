pub fn encode(plain: &str) -> String {
    plain
        .to_ascii_lowercase()
        .chars()
        .filter_map(swap)
        .collect::<Vec<char>>()
        .chunks(5)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn decode(cipher: &str) -> String {
    cipher
        .to_ascii_lowercase()
        .chars()
        .filter_map(swap)
        .collect()
}

fn swap(ch: char) -> Option<char> {
    match ch {
        '0'..='9' => Some(ch),
        'a'..='z' => Some((b'a' + b'z' - ch as u8) as char),
        _ => None,
    }
}

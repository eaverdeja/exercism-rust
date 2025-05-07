pub fn rotate(input: &str, key: u8) -> String {
    input
        .chars()
        .map(|ch| match ch {
            'a'..='z' => rotate_char(ch, key, 'a'),
            'A'..='Z' => rotate_char(ch, key, 'A'),
            _ => ch,
        })
        .collect()
}

fn rotate_char(ch: char, key: u8, base: char) -> char {
    let offset = ch as u8 - base as u8;
    let rotated = (offset + key) % 26;
    char::from(base as u8 + rotated)
}

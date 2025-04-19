pub fn reply(message: &str) -> &str {
    match message.trim() {
        m if is_yelling(m) && m.ends_with("?") => "Calm down, I know what I'm doing!",
        m if is_yelling(m) => "Whoa, chill out!",
        m if m.ends_with("?") => "Sure.",
        "" => "Fine. Be that way!",
        _ => "Whatever.",
    }
}

fn is_yelling(string: &str) -> bool {
    has_letters(string)
        && string
            .chars()
            .filter(char::is_ascii_alphabetic)
            .all(|c| c.is_ascii_uppercase())
}

fn has_letters(string: &str) -> bool {
    string.chars().any(|c| c.is_ascii_alphabetic())
}

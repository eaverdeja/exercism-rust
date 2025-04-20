use std::iter;

pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|c: char| c.is_whitespace() || c == '-' || c == '_')
        .filter(|word| !word.is_empty())
        .map(to_acronym)
        .collect()
}

fn to_acronym(word: &str) -> String {
    // Make the first letter uppercase
    let first_letter = word
        .chars()
        .next()
        .map(|c| c.to_ascii_uppercase())
        .unwrap_or_default();

    if word.chars().all(char::is_uppercase) {
        // All-caps words only get their first letter
        first_letter.to_string()
    } else {
        // Otherwise we need to collect all capital letters in the word.
        // We chain and then skip the first since it is already uppercase
        let tail = word.chars().skip(1).filter(|c| c.is_uppercase());
        iter::once(first_letter).chain(tail).collect()
    }
}

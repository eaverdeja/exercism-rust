use std::collections::HashMap;

pub fn word_count(words: &str) -> HashMap<String, u32> {
    words
        .split(|ch: char| ch.is_ascii_whitespace() || (ch != '\'' && ch.is_ascii_punctuation()))
        .fold(HashMap::new(), |mut tally, word| {
            let word = word.trim_matches('\'').to_ascii_lowercase();
            if word.is_empty() {
                return tally;
            }

            tally.entry(word).and_modify(|e| *e += 1).or_insert(1);
            tally
        })
}

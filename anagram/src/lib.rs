use std::collections::{HashMap, HashSet};

use unicode_segmentation::UnicodeSegmentation;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lower_word = word.to_lowercase();
    let word_letter_count = letter_count(&lower_word);

    possible_anagrams
        .iter()
        .filter(|&candidate| {
            let lower_candidate = candidate.to_lowercase();
            if lower_word == lower_candidate {
                return false;
            }

            word_letter_count == letter_count(&lower_candidate)
        })
        .copied()
        .collect()
}

fn letter_count(word: &str) -> HashMap<String, i32> {
    word.graphemes(true)
        .fold(HashMap::new(), |mut tally, slice| {
            tally
                .entry(slice.to_owned())
                .and_modify(|count| *count += 1)
                .or_insert(1);
            tally
        })
}

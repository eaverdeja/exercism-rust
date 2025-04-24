use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    h.iter().flat_map(|(&k, v)| to_scores(v, k)).collect()
}

fn to_scores(chars: &[char], score: i32) -> impl Iterator<Item = (char, i32)> {
    chars.iter().map(move |ch| (ch.to_ascii_lowercase(), score))
}

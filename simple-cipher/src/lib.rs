use rand::distr::{SampleString, Uniform};
enum Op {
    Encode,
    Decode,
}

pub fn encode(key: &str, s: &str) -> Option<String> {
    process(key, s, &Op::Encode)
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    process(key, s, &Op::Decode)
}

pub fn encode_random(s: &str) -> (String, String) {
    let key = generate_key();
    let encoded =
        process(key.as_str(), s, &Op::Encode).expect("key was auto-generated and should be valid");
    (key, encoded)
}

fn process(key: &str, s: &str, op: &Op) -> Option<String> {
    if key.is_empty() || !key.chars().all(|k| k.is_ascii_lowercase()) {
        return None;
    }

    Some(
        s.chars()
            .zip(key.chars().cycle())
            .map(|(ch, k)| match ch {
                'a'..='z' => rotate_char(ch, k, 'a', op),
                'A'..='Z' => rotate_char(ch, k, 'A', op),
                _ => ch,
            })
            .collect(),
    )
}

fn rotate_char(ch: char, key: char, base: char, op: &Op) -> char {
    let offset = key as u8 - base as u8;
    let normalized = ch as u8 - base as u8;
    let rotation = match op {
        Op::Encode => normalized + offset,
        Op::Decode => normalized + 26 - offset,
    };
    char::from(base as u8 + (rotation % 26))
}

fn generate_key() -> String {
    let between = Uniform::try_from('a'..='z').unwrap();
    let mut rng = rand::rng();
    between.sample_string(&mut rng, 100)
}

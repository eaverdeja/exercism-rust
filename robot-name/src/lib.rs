use rand::Rng;
use std::{
    collections::HashSet,
    sync::{LazyLock, Mutex},
};

static NAMES: LazyLock<Mutex<HashSet<String>>> = LazyLock::new(|| Mutex::new(HashSet::new()));

pub struct Robot {
    name: String,
}

impl Robot {
    pub fn new() -> Self {
        Self {
            name: generate_unique_name(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        {
            let mut names = NAMES.lock().unwrap();
            names.remove(&self.name);
        }
        self.name = generate_unique_name();
    }
}

fn generate_unique_name() -> String {
    let mut names = NAMES.lock().unwrap();

    let mut name = generate_name();
    while names.contains(&name) {
        name = generate_name();
    }
    names.insert(name.clone());

    name
}

fn generate_name() -> String {
    let random_letters = (0..2).map(|_| random_letter());
    let random_digits = (0..3).map(|_| random_digit());

    random_letters.chain(random_digits).collect()
}

fn random_digit() -> char {
    rand::rng().random_range('0'..='9')
}

fn random_letter() -> char {
    rand::rng().random_range('A'..='Z')
}

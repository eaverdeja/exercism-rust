use std::{collections::HashMap, ops::Deref};

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct Attrs(pub HashMap<String, String>);

impl From<&[(&str, &str)]> for Attrs {
    fn from(value: &[(&str, &str)]) -> Self {
        Attrs(
            value
                .iter()
                .map(|&(key, value)| (key.to_string(), value.to_string()))
                .collect(),
        )
    }
}

impl PartialEq<HashMap<String, String>> for Attrs {
    fn eq(&self, other: &HashMap<String, String>) -> bool {
        self.0 == *other
    }
}

impl Deref for Attrs {
    type Target = HashMap<String, String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Attrs {
    pub fn new() -> Self {
        Self::default()
    }
}

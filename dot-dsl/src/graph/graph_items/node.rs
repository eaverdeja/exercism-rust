use super::attrs::Attrs;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Node {
    pub value: String,
    attrs: Attrs,
}

impl Node {
    pub fn new(value: &str) -> Self {
        Node {
            value: value.to_string(),
            attrs: Attrs::new(),
        }
    }

    pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
        self.attrs = Attrs::from(attrs);
        self
    }

    pub fn attr(&self, attr: &str) -> Option<&str> {
        self.attrs.get(attr).map(|s| s.as_str())
    }
}

use super::attrs::Attrs;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Edge {
    from: String,
    to: String,
    attrs: Attrs,
}

impl Edge {
    pub fn new(from: &str, to: &str) -> Self {
        Edge {
            from: from.to_string(),
            to: to.to_string(),
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

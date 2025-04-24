use graph_items::{attrs::Attrs, edge::Edge, node::Node};

pub mod graph_items;

#[derive(Default)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub attrs: Attrs,
}

impl Graph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
        self.nodes = nodes.to_vec();
        self
    }

    pub fn with_edges(mut self, edges: &[Edge]) -> Self {
        self.edges = edges.to_vec();
        self
    }

    pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
        self.attrs = Attrs::from(attrs);
        self
    }

    pub fn node(&self, node: &str) -> Option<&Node> {
        self.nodes.iter().find(|n| n.value == node)
    }
}

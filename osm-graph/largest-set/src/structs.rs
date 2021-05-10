use std::cmp::Ordering;

use crate::constants::*;

#[derive(Debug, Clone)]
pub struct Node {
    pub osm_id: usize,
    pub latitude: String,
    pub longitude: String,
    pub height: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Edge {
    pub from: NodeId,
    pub to: NodeId,
    pub cost: Vec<String>,
    pub contracted_edges: (isize, isize),
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Edge) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Edge) -> Ordering {
        self.from.cmp(&other.from).then(self.to.cmp(&other.to))
    }
}
impl Eq for Edge {}

impl Edge {
    pub fn new(
        from: NodeId,
        to: NodeId,
        cost: Vec<String>,
        contracted_edges: (isize, isize),
    ) -> Self {
        Edge {
            from,
            to,
            cost,
            contracted_edges,
        }
    }
}

use std::cmp::Ordering;

use crate::constants::*;

#[derive(Debug)]
pub struct Node {
    pub latitude: f32,
    pub longitude: f32,
    pub partition: PartitionId,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Edge {
    pub from: NodeId,
    pub to: NodeId,
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

impl Edge {
    pub fn new(from: NodeId, to: NodeId) -> Self {
        Edge { from, to }
    }
}

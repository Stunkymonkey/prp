use serde::Serialize;
use std::cmp::Ordering;

use crate::constants::*;

#[derive(Serialize, Debug, Clone)]
pub struct Node {
    pub latitude: f32,
    pub longitude: f32,
    pub cluster: ClusterId,
}

#[derive(Serialize, Debug, Clone, Copy, Eq, PartialEq)]
pub struct Edge {
    pub source: NodeId,
    pub target: NodeId,
    #[serde(skip_serializing)]
    pub id: Option<EdgeId>,
    pub contrated_previous: Option<EdgeId>,
    pub contrated_next: Option<EdgeId>,
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Edge) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Edge) -> Ordering {
        self.source
            .cmp(&other.source)
            .then(self.target.cmp(&other.target))
            .then(self.contrated_previous.cmp(&other.contrated_previous))
            .then(self.contrated_next.cmp(&other.contrated_next))
    }
}

impl Edge {
    pub fn new(from: NodeId, to: NodeId) -> Self {
        Edge {
            source: from,
            target: to,
            id: None,
            contrated_previous: None,
            contrated_next: None,
        }
    }
}

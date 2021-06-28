use crate::constants::*;

#[derive(Debug, Clone)]
pub struct Node {
    pub latitude: f64,
    pub longitude: f64,
    pub partition: PartitionId,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Edge {
    pub source: NodeId,
    pub target: NodeId,
}

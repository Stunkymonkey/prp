use serde::Deserialize;
use std::cmp::Ordering;

use crate::constants::*;

#[derive(Deserialize, Clone, Debug)]
pub struct Node {
    pub latitude: f32,
    pub longitude: f32,
    pub rank: Rank,
    pub layer_height: LayerHeight,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Edge {
    pub source: NodeId,
    pub target: NodeId,
    pub weight: Vec<Weight>,
    pub contrated_edges: Option<(EdgeId, EdgeId)>,
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
    }
}
impl Eq for Edge {}

impl Edge {
    pub fn new(from: NodeId, to: NodeId, weight: Vec<Weight>) -> Self {
        Edge {
            source: from,
            target: to,
            weight,
            contrated_edges: None,
        }
    }
    #[allow(dead_code)]
    pub fn test(from: NodeId, to: NodeId, weight: Vec<Weight>) -> Self {
        Edge {
            source: from,
            target: to,
            weight,
            contrated_edges: None,
        }
    }
    #[allow(dead_code)]
    pub fn shortcut(
        from: NodeId,
        to: NodeId,
        weight: Vec<Weight>,
        previous: NodeId,
        next: NodeId,
    ) -> Self {
        Edge {
            source: from,
            target: to,
            weight,
            contrated_edges: Some((previous, next)),
        }
    }
}

#[derive(Deserialize, Clone)]
pub struct GridBounds {
    pub lat_min: f32,
    pub lat_max: f32,
    pub lng_min: f32,
    pub lng_max: f32,
}

#[derive(Deserialize, Clone)]
pub struct BinFile {
    pub nodes: Vec<Node>,
    pub up_offset: Vec<EdgeId>,
    pub down_offset: Vec<EdgeId>,
    pub down_index: Vec<EdgeId>,
    pub edges: Vec<Edge>,
    pub grid_offset: Vec<GridId>,
    pub grid: Vec<NodeId>,
    pub grid_bounds: GridBounds,
    pub metrics: Vec<String>,
}

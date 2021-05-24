use serde::Serialize;
use std::cmp::Ordering;

use crate::constants::*;

#[derive(Serialize, Debug)]
pub struct Node {
    pub latitude: Angle,
    pub longitude: Angle,
    pub rank: Rank,
    pub partition: PartitionId,
    pub layer_height: LayerHeight,
    #[serde(skip_serializing)]
    pub old_id: Option<NodeId>,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct Edge {
    pub from: NodeId,
    pub to: NodeId,
    #[serde(skip_serializing)]
    pub id: Option<EdgeId>,
    #[serde(skip_serializing)]
    pub cost: Vec<Cost>,
    pub contracted_edges: Option<(EdgeId, EdgeId)>,
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
    pub fn new(from: NodeId, to: NodeId, cost: Vec<Cost>) -> Self {
        Edge {
            from,
            to,
            id: None,
            cost,
            contracted_edges: None,
        }
    }
    #[allow(dead_code)]
    pub fn test(from: NodeId, to: NodeId, cost: Vec<Cost>, id: NodeId) -> Self {
        Edge {
            from,
            to,
            cost,
            id: Some(id),
            contracted_edges: None,
        }
    }
    #[allow(dead_code)]
    pub fn shortcut(
        from: NodeId,
        to: NodeId,
        cost: Vec<Cost>,
        id: NodeId,
        contracted_edges: (EdgeId, EdgeId),
    ) -> Self {
        Edge {
            from,
            to,
            cost,
            id: Some(id),
            contracted_edges: Some(contracted_edges),
        }
    }
}

#[derive(Serialize)]
pub struct GridBounds {
    pub lat_amount: usize,
    pub lat_min: Angle,
    pub lat_max: Angle,
    pub lng_amount: usize,
    pub lng_min: Angle,
    pub lng_max: Angle,
}

#[derive(Serialize)]
pub struct BinFile {
    pub nodes: Vec<Node>,
    pub mlp_layers: Vec<usize>,
    pub edges: Vec<Edge>,
    pub edge_costs: Vec<Cost>,
    pub up_offset: Vec<EdgeId>,
    pub down_offset: Vec<EdgeId>,
    pub down_index: Vec<EdgeId>,
    pub grid_offset: Vec<GridId>,
    pub grid: Vec<NodeId>,
    pub grid_bounds: GridBounds,
    pub metrics: Vec<String>,
}

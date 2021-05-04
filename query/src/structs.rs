use serde::Deserialize;

use crate::constants::*;
use crate::graph::Graph;

#[derive(Deserialize, Clone, Debug)]
pub struct Node {
    pub latitude: f32,
    pub longitude: f32,
    pub rank: Rank,
    pub layer_height: LayerHeight,
}

#[derive(Deserialize, Debug, Copy, Clone, PartialEq)]
pub struct Edge {
    pub from: NodeId,
    pub to: NodeId,
    pub contrated_edges: Option<(EdgeId, EdgeId)>,
}

impl Edge {
    /// get to field
    pub fn get_to(&self) -> NodeId {
        self.to
    }

    /// get to field
    pub fn get_from(&self) -> NodeId {
        self.from
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

#[derive(Clone)]
pub struct WebData {
    pub nodes: Vec<Node>,
    pub graph: Graph,
    pub grid_offset: Vec<GridId>,
    pub grid: Vec<NodeId>,
    pub grid_bounds: GridBounds,
    pub metrics: Vec<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct EvalFile {
    pub points: EvalPoint,
}

#[derive(Deserialize, Clone, Debug)]
pub struct EvalPoint {
    pub orig_start_id: Option<NodeId>,
    pub orig_end_id: Option<NodeId>,
    pub start_pos: Vec<f64>,
    pub end_pos: Vec<f64>,
    pub alpha: Vec<f64>,
}

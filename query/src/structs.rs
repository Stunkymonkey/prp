use serde::Deserialize;
use std::str::FromStr;

use crate::constants::*;
use crate::graph::Graph;

#[derive(Deserialize, Clone, Debug)]
pub struct Node {
    pub latitude: Angle,
    pub longitude: Angle,
    pub rank: Rank,
    pub partition: PartitionId,
}

#[derive(Deserialize, Debug, Copy, Clone, PartialEq)]
pub struct Edge {
    pub from: NodeId,
    pub to: NodeId,
    pub level: Option<Level>,
    pub contracted_edges: Option<(EdgeId, EdgeId)>,
    #[serde(skip_deserializing)]
    pub old_id: Option<EdgeId>,
}

impl Edge {
    /// get to field
    pub fn get_to(&self) -> NodeId {
        self.to
    }

    /// get from field
    pub fn get_from(&self) -> NodeId {
        self.from
    }
}

#[derive(Deserialize, Clone)]
pub struct GridBounds {
    pub lat_amount: usize,
    pub lat_min: Angle,
    pub lat_max: Angle,
    pub lng_amount: usize,
    pub lng_min: Angle,
    pub lng_max: Angle,
}

#[derive(Deserialize, Clone)]
pub struct BinFile {
    pub nodes: Vec<Node>,
    pub mlp_levels: Vec<usize>,
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
    pub mlp_levels: Vec<usize>,
    pub graph: Graph,
    pub grid_offset: Vec<GridId>,
    pub grid: Vec<NodeId>,
    pub grid_bounds: GridBounds,
    pub metrics: Vec<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Location {
    pub latitude: Angle,
    pub longitude: Angle,
}

#[derive(Deserialize, Clone, Debug)]
pub struct EvalPoint {
    pub id: usize,
    pub orig_start_id: Option<NodeId>,
    pub orig_end_id: Option<NodeId>,
    pub start_id: Option<NodeId>,
    pub end_id: Option<NodeId>,
    pub start_pos: Location,
    pub end_pos: Location,
    pub alpha: Vec<f64>,
}

#[derive(Debug, Copy, Clone)]
pub enum QueryType {
    Normal,
    Bi,
    Pch,
    Pcrp,
    Prp,
}

impl FromStr for QueryType {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "normal" => Ok(QueryType::Normal),
            "bi" => Ok(QueryType::Bi),
            "pch" => Ok(QueryType::Pch),
            "pcrp" => Ok(QueryType::Pcrp),
            "prp" => Ok(QueryType::Prp),
            _ => Err("no match"),
        }
    }
}

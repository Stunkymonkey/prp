pub type NodeId = usize;
pub type EdgeId = usize;
pub type Cost = f64;
pub type Rank = usize;
pub type PartitionId = usize;
pub type LayerHeight = usize;
pub type GridId = usize;
pub type Angle = f64;

#[allow(dead_code)]
pub const INVALID_NODE: NodeId = std::usize::MAX;
#[allow(dead_code)]
pub const INVALID_EDGE: NodeId = std::usize::MAX;
#[allow(dead_code)]
pub const COST_MAX: Cost = std::f64::MAX;
pub const INVALID_RANK: Rank = std::usize::MAX;
pub const INVALID_LAYER_HEIGHT: LayerHeight = std::usize::MAX;

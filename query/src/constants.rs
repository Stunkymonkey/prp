pub type NodeId = usize;
pub type EdgeId = usize;
pub type Cost = f64;
pub type Rank = usize;
pub type PartitionId = usize;
pub type Level = usize;
pub type GridId = usize;
pub type Angle = f64;

#[allow(dead_code)]
pub const INVALID_NODE: NodeId = usize::MAX;
#[allow(dead_code)]
pub const INVALID_EDGE: NodeId = usize::MAX;
#[allow(dead_code)]
pub const COST_MAX: Cost = f64::MAX;
pub const INVALID_RANK: Rank = usize::MAX;
pub const INVALID_LEVEL: Level = usize::MAX;

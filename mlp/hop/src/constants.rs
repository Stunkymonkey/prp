pub type NodeId = usize;
pub type EdgeId = usize;
pub type Cost = usize;
pub type PartitionId = usize;

#[allow(dead_code)]
pub const INVALID_NODE: NodeId = std::usize::MAX;
#[allow(dead_code)]
pub const INVALID_EDGE: NodeId = std::usize::MAX;
pub const INVALID_PARTITION: PartitionId = std::usize::MAX;
pub const COST_MAX: Cost = std::usize::MAX;

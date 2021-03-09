pub type NodeId = usize;
pub type EdgeId = usize;
pub type Weight = f64;
// pub type Rank = usize;
pub type ClusterId = usize;

#[allow(dead_code)]
pub const INVALID_NODE: NodeId = std::usize::MAX;
#[allow(dead_code)]
pub const INVALID_EDGE: NodeId = std::usize::MAX;
// pub const WEIGHT_MAX: Weight = std::usize::MAX;
// pub const INVALID_RANK: Rank = std::usize::MAX;
pub const INVALID_CLUSTER: ClusterId = std::usize::MAX;

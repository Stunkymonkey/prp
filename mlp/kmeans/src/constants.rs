pub type NodeId = usize;
#[allow(dead_code)]
pub type EdgeId = usize;
pub type ClusterId = usize;

#[allow(dead_code)]
pub const INVALID_NODE: NodeId = std::usize::MAX;
#[allow(dead_code)]
pub const INVALID_EDGE: NodeId = std::usize::MAX;
pub const INVALID_CLUSTER: ClusterId = std::usize::MAX;
pub type NodeId = usize;
pub type EdgeId = usize;
pub type Weight = f64;
pub type Rank = usize;
pub type LayerHeight = usize;
pub type GridId = usize;

#[allow(dead_code)]
pub const INVALID_NODE: NodeId = std::usize::MAX;
#[allow(dead_code)]
pub const INVALID_EDGE: NodeId = std::usize::MAX;
#[allow(dead_code)]
pub const WEIGHT_MAX: Weight = std::f64::MAX;
pub const INVALID_RANK: Rank = std::usize::MAX;
pub const INVALID_LAYER_HEIGHT: LayerHeight = std::usize::MAX;
// ratio: north south 876km / west east 640 km ~ 100:136
pub const LAT_GRID_AMOUNT: usize = 136;
pub const LNG_GRID_AMOUNT: usize = 100;

use super::*;
use ordered_float::OrderedFloat;
use std::cmp::Ordering;

#[derive(Copy, Clone)]
pub struct MinHeapItem {
    pub node: NodeId,
    pub dist: Cost,
    pub prev_edge: Option<EdgeId>,
}

// Manually implement Ord so we get a min-heap instead of a max-heap
impl MinHeapItem {
    pub fn new(node: NodeId, dist: Cost, prev_edge: Option<EdgeId>) -> MinHeapItem {
        MinHeapItem {
            node,
            dist,
            prev_edge,
        }
    }
}

impl PartialOrd for MinHeapItem {
    fn partial_cmp(&self, other: &MinHeapItem) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// reverse, so the maxheap becomes a min-heap
impl Ord for MinHeapItem {
    fn cmp(&self, other: &MinHeapItem) -> Ordering {
        OrderedFloat(self.dist)
            .cmp(&OrderedFloat(other.dist))
            .reverse()
    }
}

impl PartialEq for MinHeapItem {
    fn eq(&self, other: &MinHeapItem) -> bool {
        self.dist == other.dist
    }
}

impl Eq for MinHeapItem {}

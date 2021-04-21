use super::*;
use ordered_float::OrderedFloat;
use std::cmp::Ordering;

#[derive(Copy, Clone, Debug)]
pub struct MaxHeapItem {
    pub partition_ids: (PartitionId, PartitionId),
    pub amount_merged: (usize, usize),
    pub heuristic: f64,
}

// Manually implement Ord so we get a min-heap instead of a max-heap
impl MaxHeapItem {
    pub fn new(
        partition_ids: (PartitionId, PartitionId),
        amount_merged: (usize, usize),
        heuristic: f64,
    ) -> MaxHeapItem {
        MaxHeapItem {
            partition_ids,
            amount_merged,
            heuristic,
        }
    }
}

impl PartialOrd for MaxHeapItem {
    fn partial_cmp(&self, other: &MaxHeapItem) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MaxHeapItem {
    fn cmp(&self, other: &MaxHeapItem) -> Ordering {
        OrderedFloat(self.heuristic).cmp(&OrderedFloat(other.heuristic))
    }
}

impl PartialEq for MaxHeapItem {
    fn eq(&self, other: &MaxHeapItem) -> bool {
        self.heuristic == other.heuristic
    }
}

impl Eq for MaxHeapItem {}

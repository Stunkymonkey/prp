use super::*;
use min_heap::*;
use std::collections::BinaryHeap;
use valid_flag::*;

#[derive(Clone)]
pub struct Dijkstra {
    dist_up: Vec<(NodeId, Option<Weight>)>,
    dist_down: Vec<(NodeId, Option<Weight>)>,
    visited_up: ValidFlag,
    visited_down: ValidFlag,
    heap_up: BinaryHeap<MinHeapItem>,
    heap_down: BinaryHeap<MinHeapItem>,
}

impl Dijkstra {
    /// general constructor
    pub fn new(amount_nodes: usize) -> Self {
        Dijkstra {
            dist_up: vec![(std::usize::MAX, None); amount_nodes],
            dist_down: vec![(std::usize::MAX, None); amount_nodes],
            visited_up: ValidFlag::new(amount_nodes),
            visited_down: ValidFlag::new(amount_nodes),
            heap_up: BinaryHeap::new(),
            heap_down: BinaryHeap::new(),
        }
    }

    /// find path from start to end
    #[allow(clippy::too_many_arguments)]
    pub fn find_path(
        &mut self,
        _start: NodeId,
        _end: NodeId,
        _nodes: &[Node],
        _edges: &[Edge],
        _up_offset: &[EdgeId],
        _down_offset: &[EdgeId],
        _down_index: &[EdgeId],
    ) -> Option<(Vec<NodeId>, f32)> {
        Some((vec![42], 666.666))
    }
}

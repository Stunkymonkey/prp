// based on https://rosettacode.org/wiki/Dijkstra%27s_algorithm#Rust

use super::*;
use min_heap::*;
use std::collections::{BTreeSet, BinaryHeap};
use valid_flag::*;

#[derive(Clone)]
pub struct HopDijkstra {
    dist: Vec<(Cost, Option<NodeId>)>,
    heap: BinaryHeap<MinHeapItem>,
    visited: ValidFlag,
}

impl HopDijkstra {
    /// general constructor
    pub fn new(amount_nodes: usize) -> Self {
        let dist = vec![(COST_MAX, None); amount_nodes];
        let heap = BinaryHeap::new();
        let visited = ValidFlag::new(amount_nodes);
        HopDijkstra {
            dist,
            heap,
            visited,
        }
    }

    /// reseting its internal state
    pub fn reset_state(&mut self) {
        self.heap.clear();
        self.visited.invalidate_all();
    }

    /// return cost to reach nodes
    pub fn get_costs(
        &mut self,
        from: NodeId,
        edges: &[Edge],
        offset: &[EdgeId],
        current_set: &BTreeSet<NodeId>,
        distances: &mut Vec<(Cost, NodeId)>,
    ) {
        self.reset_state();

        self.heap.push(MinHeapItem::new(from, 0, None));

        while let Some(MinHeapItem {
            node,
            dist,
            prev_edge,
        }) = self.heap.pop()
        {
            // node has already been visited and can be skipped
            // replacement for decrease key operation
            if self.visited.is_valid(node) && dist > self.dist[node].0 {
                continue;
            }

            self.visited.set_valid(node);
            self.dist[node] = (dist, prev_edge);

            if distances[node].0 > dist {
                distances[node] = (dist, from)
            }

            for edge in graph_helper::get_up_edge_ids(node, &offset) {
                let new_edge = &edges[edge];

                // skip edges, that go outside of the parent-set
                if !current_set.contains(&new_edge.to) {
                    continue;
                }

                let alt = dist + 1;
                if !self.visited.is_valid(new_edge.to) || alt < self.dist[new_edge.to].0 {
                    self.heap
                        .push(MinHeapItem::new(new_edge.to, alt, Some(edge)));
                }
            }
        }
    }
}

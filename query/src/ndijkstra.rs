// based on https://rosettacode.org/wiki/Dijkstra%27s_algorithm#Rust

use super::*;
use mch::{costs_by_alpha, equal_weights, same_array};
use min_heap::*;
use std::collections::BinaryHeap;
use valid_flag::*;

#[derive(Clone)]
pub struct NDijkstra {
    dist: Vec<(Cost, Option<NodeId>)>,
    heap: BinaryHeap<MinHeapItem>,
    visited: ValidFlag,
    last_from: NodeId,
    last_pref: Vec<Cost>,
}

impl NDijkstra {
    /// general constructor
    pub fn new(amount_nodes: usize, dim: usize) -> Self {
        let dist = vec![(COST_MAX, None); amount_nodes];
        let heap = BinaryHeap::new();
        let visited = ValidFlag::new(amount_nodes);
        NDijkstra {
            dist,
            heap,
            visited,
            last_from: INVALID_NODE,
            last_pref: equal_weights(dim),
        }
    }

    /// reseting its internal state
    pub fn reset_state(&mut self) {
        self.last_from = INVALID_NODE;
        self.heap.clear();
        self.visited.invalidate_all();
    }

    /// return path of edges(!) from source to target not path of nodes!
    pub fn find_path(
        &mut self,
        from: NodeId,
        to: NodeId,
        alpha: Vec<f64>,
        graph: &Graph,
    ) -> Option<(Vec<NodeId>, Cost)> {
        if self.last_from == from && same_array(&self.last_pref, &alpha) {
            if self.visited.is_valid(to) {
                return Some(self.resolve_path(to, &graph.edges));
            }
        } else {
            // If something changed, we initialize it normally
            self.reset_state();
            self.last_from = from;
            self.last_pref = alpha.clone();

            self.heap.push(MinHeapItem::new(from, 0.0, None));
        }

        while let Some(MinHeapItem {
            node,
            cost,
            prev_edge,
        }) = self.heap.pop()
        {
            // node has already been visited and can be skipped
            // replacement for decrease key operation
            if self.visited.is_valid(node) && cost > self.dist[node].0 {
                continue;
            }

            self.visited.set_valid(node);
            self.dist[node] = (cost, prev_edge);

            for edge in graph.get_up_edge_ids(node) {
                let new_edge = graph.get_edge(edge);

                // skip edges, that are shortcuts
                if new_edge.contrated_edges.is_some() {
                    continue;
                }

                let alt = cost + costs_by_alpha(&graph.get_edge_costs(edge), &alpha);
                if !self.visited.is_valid(new_edge.to) || alt < self.dist[new_edge.to].0 {
                    self.heap
                        .push(MinHeapItem::new(new_edge.to, alt, Some(edge)));
                }
            }

            // found end
            if node == to {
                return Some(self.resolve_path(to, &graph.edges));
            }
        }
        None
    }

    /// recreate path backwards
    fn resolve_path(&self, end: NodeId, edges: &[Edge]) -> (Vec<NodeId>, Cost) {
        let weight = self.dist[end].0;
        let mut path = Vec::with_capacity(self.dist.len() / 2);
        let mut current_dist = self.dist[end];
        while let Some(prev) = current_dist.1 {
            path.push(prev);
            current_dist = self.dist[edges[prev].from];
        }
        path.reverse();
        (path, weight)
    }
}

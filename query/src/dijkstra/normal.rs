// based on https://rosettacode.org/wiki/Dijkstra%27s_algorithm#Rust

use super::*;
use mch::costs_by_alpha;
use min_heap::*;
use std::collections::BinaryHeap;
use valid_flag::*;

#[derive(Clone)]
pub struct Dijkstra<E: Export> {
    dist: Vec<(Cost, Option<NodeId>)>,
    heap: BinaryHeap<MinHeapItem>,
    visited: ValidFlag,
    debug: bool,
    pub exporter: E,
}

impl<E: Export> FindPath<E> for Dijkstra<E> {
    /// general constructor
    fn new(amount_nodes: usize, exporter: E) -> Self {
        let dist = vec![(COST_MAX, None); amount_nodes];
        let heap = BinaryHeap::new();
        let visited = ValidFlag::new(amount_nodes);
        Dijkstra {
            dist,
            heap,
            visited,
            debug: false,
            exporter,
        }
    }

    /// reseting its internal state
    fn reset_state(&mut self) {
        self.heap.clear();
        self.visited.invalidate_all();
    }
    fn get_query_export(&self) -> &E {
        &self.exporter
    }

    /// return path of edges(!) from source to target not path of nodes!
    fn find_path(
        &mut self,
        from: NodeId,
        to: NodeId,
        alpha: Vec<f64>,
        graph: &Graph,
        _nodes: &[Node],
        _mlp_levels: &[usize],
    ) -> Option<(Vec<NodeId>, Cost)> {
        self.reset_state();

        self.heap.push(MinHeapItem::new(from, 0.0, None));

        while let Some(MinHeapItem {
            node,
            cost,
            prev_edge,
        }) = self.heap.pop()
        {
            self.exporter.heap_pop();
            // node has already been visited and can be skipped
            // replacement for decrease key operation
            if self.visited.is_valid(node) && cost > self.dist[node].0 {
                continue;
            }

            self.visited.set_valid(node);
            self.dist[node] = (cost, prev_edge);

            self.exporter.visited_node(node);
            self.exporter.visited_edge(prev_edge);

            // found end
            if node == to {
                return Some(self.resolve_path(to, &graph.edges));
            }

            for edge_id in graph.get_up_edge_ids(node) {
                let new_edge = graph.get_edge(edge_id);

                // skip edges, that are shortcuts
                if new_edge.contracted_edges.is_some() {
                    if self.debug {
                        continue;
                    } else {
                        break;
                    }
                }

                self.exporter.relaxed_edge();

                let alt = cost + costs_by_alpha(graph.get_edge_costs(edge_id), &alpha);
                if !self.visited.is_valid(new_edge.to) || alt < self.dist[new_edge.to].0 {
                    self.heap
                        .push(MinHeapItem::new(new_edge.to, alt, Some(edge_id)));
                }
            }
        }
        None
    }
}
impl<E: Export> Dijkstra<E> {
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
    pub fn set_debug(&mut self, debug: bool) {
        self.debug = debug;
    }
}

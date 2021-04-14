// based on https://rosettacode.org/wiki/Dijkstra%27s_algorithm#Rust

use super::*;
use mch::{add_edge_costs, costs_by_alpha, equal_weights, same_array};
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
        offset: &[EdgeId],
        edges: &[Edge],
    ) -> Option<(Vec<NodeId>, Vec<Cost>)> {
        if self.last_from == from && same_array(&self.last_pref, &alpha) {
            if self.visited.is_valid(to) {
                return Some(self.resolve_path(to, &edges));
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
            dist,
            prev_edge,
        }) = self.heap.pop()
        {
            // node has already been visited and can be skipped
            // replacement for decrease key operation
            if self.visited.is_valid(node) && dist >= self.dist[node].0 {
                continue;
            }

            self.visited.set_valid(node);
            self.dist[node] = (dist, prev_edge);

            for edge in graph_helper::get_up_edge_ids(node, &offset) {
                let new_edge = &edges[edge];
                let alt = dist + costs_by_alpha(&new_edge.cost, &alpha);
                if !self.visited.is_valid(new_edge.to) || alt < self.dist[new_edge.to].0 {
                    self.heap
                        .push(MinHeapItem::new(new_edge.to, alt, Some(edge)));
                }
            }

            // found end
            if node == to {
                return Some(self.resolve_path(to, &edges));
            }
        }
        None
    }

    /// recreate path backwards
    fn resolve_path(&self, end: NodeId, edges: &[Edge]) -> (Vec<NodeId>, Vec<Cost>) {
        let mut path = Vec::with_capacity(self.dist.len() / 2);
        let mut total_dimension_costs = vec![0.0; edges[0].cost.len()];
        let mut current_dist = self.dist[end];
        while let Some(prev) = current_dist.1 {
            path.push(prev);
            current_dist = self.dist[edges[prev].from];
            add_edge_costs(&mut total_dimension_costs, &edges[prev].cost);
        }
        path.reverse();
        (path, total_dimension_costs)
    }
}

#[test]
fn no_path() {
    // Start: 1
    // Goal: 0
    //
    // 0->1->2

    let amount_nodes = 3;

    let mut edges = Vec::<Edge>::new();
    edges.push(Edge::new(0, 1, vec![1.0]));
    edges.push(Edge::new(1, 2, vec![1.0]));

    let mut up_offset = Vec::<EdgeId>::new();
    let mut down_offset = Vec::<EdgeId>::new();
    offset::generate_offsets(&mut edges, &mut up_offset, &mut down_offset, amount_nodes);
    let mut d = NDijkstra::new(amount_nodes, 1);
    let result = d.find_path(1, 0, vec![1.0], &up_offset, &edges);

    assert!(result.is_none());
}

#[test]
fn simple_path() {
    // Start: 0
    // Goal: 2
    //
    // 0-1->1-2->2
    //      |
    //      1
    //      |
    //      V
    //      3

    let amount_nodes = 4;

    let mut edges = Vec::<Edge>::new();
    edges.push(Edge::new(0, 1, vec![1.0]));
    edges.push(Edge::new(1, 2, vec![2.0]));
    edges.push(Edge::new(1, 3, vec![1.0]));

    let mut up_offset = Vec::<EdgeId>::new();
    let mut down_offset = Vec::<EdgeId>::new();
    offset::generate_offsets(&mut edges, &mut up_offset, &mut down_offset, amount_nodes);
    let mut d = NDijkstra::new(amount_nodes, 1);
    let result = d.find_path(0, 2, vec![1.0], &up_offset, &edges);

    assert!(result.is_some());
    let path = result.unwrap();
    assert_eq!(path.0, [0, 1]);
    assert_eq!(path.1, vec![3.0]);
}

#[test]
fn shortest_path() {
    // Start: 0
    // Goal: 2
    //
    // 0-9->1-9->2
    // |         A
    // 1         |
    // |         1
    // V         |
    // 3-1->4-1->5

    let amount_nodes = 6;

    let mut edges = Vec::<Edge>::new();
    edges.push(Edge::new(0, 1, vec![9.0]));
    edges.push(Edge::new(1, 2, vec![9.0]));
    edges.push(Edge::new(0, 3, vec![1.0]));
    edges.push(Edge::new(3, 4, vec![1.0]));
    edges.push(Edge::new(4, 5, vec![1.0]));
    edges.push(Edge::new(5, 2, vec![1.0]));

    let mut up_offset = Vec::<EdgeId>::new();
    let mut down_offset = Vec::<EdgeId>::new();
    offset::generate_offsets(&mut edges, &mut up_offset, &mut down_offset, amount_nodes);
    let mut d = NDijkstra::new(amount_nodes, 1);
    let result = d.find_path(0, 2, vec![1.0], &up_offset, &edges);

    assert!(result.is_some());
    let path = result.unwrap();
    assert_eq!(path.0, [1, 3, 4, 5]);
    assert_eq!(path.1, vec![4.0]);
}

#[test]
fn simple_line() {
    // 0->1->2->3
    let amount_nodes = 4;

    let mut edges = Vec::<Edge>::new();
    edges.push(Edge::new(0, 1, vec![1.0]));
    edges.push(Edge::new(1, 2, vec![1.0]));
    edges.push(Edge::new(2, 3, vec![1.0]));

    let mut up_offset = Vec::<EdgeId>::new();
    let mut down_offset = Vec::<EdgeId>::new();
    offset::generate_offsets(&mut edges, &mut up_offset, &mut down_offset, amount_nodes);
    let mut d = NDijkstra::new(amount_nodes, 1);

    let result = d.find_path(3, 0, vec![1.0], &up_offset, &edges);
    assert!(result.is_none());

    let result = d.find_path(0, 3, vec![1.0], &up_offset, &edges);
    assert!(result.is_some());
    let path = result.unwrap();
    assert_eq!(path.0, [0, 1, 2]);
    assert_eq!(path.1, vec![3.0]);
}

#[test]
fn twice() {
    // 0->1->2
    let amount_nodes = 3;

    let mut edges = Vec::<Edge>::new();
    edges.push(Edge::new(0, 1, vec![1.0]));
    edges.push(Edge::new(1, 2, vec![1.0]));

    let mut up_offset = Vec::<EdgeId>::new();
    let mut down_offset = Vec::<EdgeId>::new();
    offset::generate_offsets(&mut edges, &mut up_offset, &mut down_offset, amount_nodes);
    let mut d = NDijkstra::new(amount_nodes, 1);

    let result = d.find_path(0, 2, vec![1.0], &up_offset, &edges);
    assert!(result.is_some());
    let path = result.unwrap();
    assert_eq!(path.0, [0, 1]);
    assert_eq!(path.1, vec![2.0]);

    let result = d.find_path(0, 2, vec![1.0], &up_offset, &edges);
    assert!(result.is_some());
    let path = result.unwrap();
    assert_eq!(path.0, [0, 1]);
    assert_eq!(path.1, vec![2.0]);

    let result = d.find_path(0, 1, vec![1.0], &up_offset, &edges);
    assert!(result.is_some());
    let path = result.unwrap();
    assert_eq!(path.0, [0]);
    assert_eq!(path.1, vec![1.0]);

    let result = d.find_path(0, 1, vec![1.0], &up_offset, &edges);
    assert!(result.is_some());
    let path = result.unwrap();
    assert_eq!(path.0, [0]);
    assert_eq!(path.1, vec![1.0]);

    let result = d.find_path(0, 2, vec![1.0], &up_offset, &edges);
    assert!(result.is_some());
    let path = result.unwrap();
    assert_eq!(path.0, [0, 1]);
    assert_eq!(path.1, vec![2.0]);
}

#[test]
fn multiple_paths() {
    //      7 -> 8 -> 9
    //      |         |
    // 0 -> 5 -> 6 -  |
    // |         |  \ |
    // 1 -> 2 -> 3 -> 4

    let amount_nodes = 10;

    let mut edges = Vec::<Edge>::new();
    edges.push(Edge::new(0, 1, vec![1.0]));
    edges.push(Edge::new(1, 2, vec![1.0]));
    edges.push(Edge::new(2, 3, vec![1.0]));
    edges.push(Edge::new(3, 4, vec![20.0]));
    edges.push(Edge::new(0, 5, vec![5.0]));
    edges.push(Edge::new(5, 6, vec![1.0]));
    edges.push(Edge::new(6, 4, vec![20.0]));
    edges.push(Edge::new(6, 3, vec![20.0]));
    edges.push(Edge::new(5, 7, vec![5.0]));
    edges.push(Edge::new(7, 8, vec![1.0]));
    edges.push(Edge::new(8, 9, vec![1.0]));
    edges.push(Edge::new(9, 4, vec![1.0]));

    let mut up_offset = Vec::<EdgeId>::new();
    let mut down_offset = Vec::<EdgeId>::new();
    offset::generate_offsets(&mut edges, &mut up_offset, &mut down_offset, amount_nodes);
    let mut d = NDijkstra::new(amount_nodes, 1);

    let result = d.find_path(4, 0, vec![1.0], &up_offset, &edges);
    assert!(result.is_none());

    let result = d.find_path(4, 4, vec![1.0], &up_offset, &edges);
    assert!(result.is_some());
    let path = result.unwrap();
    assert_eq!(path.0.len(), 0);
    assert_eq!(path.0, []);
    assert_eq!(path.1, vec![0.0]);

    let result = d.find_path(6, 3, vec![1.0], &up_offset, &edges);
    assert!(result.is_some());
    let path = result.unwrap();
    assert_eq!(path.0, [7]);
    assert_eq!(path.1, vec![20.0]);

    let result = d.find_path(1, 4, vec![1.0], &up_offset, &edges);
    assert!(result.is_some());
    let path = result.unwrap();
    assert_eq!(path.0, [2, 3, 4]);
    assert_eq!(path.1, vec![22.0]);

    let result = d.find_path(0, 4, vec![1.0], &up_offset, &edges);
    assert!(result.is_some());
    let path = result.unwrap();
    assert_eq!(path.0, [1, 6, 9, 10, 11]);
    assert_eq!(path.1, vec![13.0]);
}

#[test]
fn ndim() {
    // Start: 0
    // Goal: 5
    //
    // 0-->1-->2
    // |       |
    // V       V
    // 3-->4-->5

    let amount_nodes = 6;

    let mut edges = Vec::<Edge>::new();
    edges.push(Edge::new(0, 1, vec![9.0, 1.0]));
    edges.push(Edge::new(0, 3, vec![1.0, 9.0]));
    edges.push(Edge::new(1, 2, vec![9.0, 1.0]));
    edges.push(Edge::new(2, 5, vec![9.0, 1.0]));
    edges.push(Edge::new(3, 4, vec![1.0, 9.0]));
    edges.push(Edge::new(4, 5, vec![1.0, 9.0]));

    let mut up_offset = Vec::<EdgeId>::new();
    let mut down_offset = Vec::<EdgeId>::new();
    offset::generate_offsets(&mut edges, &mut up_offset, &mut down_offset, amount_nodes);
    let mut d = NDijkstra::new(amount_nodes, 2);

    let result = d.find_path(0, 5, vec![1.0, 0.0], &up_offset, &edges);
    assert!(result.is_some());
    let path = result.unwrap();
    assert_eq!(path.0, [1, 4, 5]);
    assert_eq!(path.1, vec![3.0, 27.0]);

    let result = d.find_path(0, 5, vec![0.0, 1.0], &up_offset, &edges);
    assert!(result.is_some());
    let path = result.unwrap();
    assert_eq!(path.0, [0, 2, 3]);
    assert_eq!(path.1, vec![27.0, 3.0]);
}

// based on https://rosettacode.org/wiki/Dijkstra%27s_algorithm#Rust

use super::*;
use min_heap::*;
use valid_flag::*;

use mch::costs_by_alpha;
use std::collections::BinaryHeap;

#[derive(Clone)]
pub struct Dijkstra<E: Export> {
    dist_up: Vec<(Cost, Option<NodeId>)>,
    dist_down: Vec<(Cost, Option<NodeId>)>,
    visited_up: ValidFlag,
    visited_down: ValidFlag,
    heap_up: BinaryHeap<MinHeapItem>,
    heap_down: BinaryHeap<MinHeapItem>,
    pub exporter: E,
}

impl<E: Export> FindPath<E> for Dijkstra<E> {
    /// general constructor
    fn new(amount_nodes: usize, exporter: E) -> Self {
        let dist_up = vec![(COST_MAX, None); amount_nodes];
        let dist_down = vec![(COST_MAX, None); amount_nodes];
        let visited_up = ValidFlag::new(amount_nodes);
        let visited_down = ValidFlag::new(amount_nodes);
        let heap_up = BinaryHeap::with_capacity(amount_nodes);
        let heap_down = BinaryHeap::with_capacity(amount_nodes);
        Dijkstra {
            dist_up,
            dist_down,
            visited_up,
            visited_down,
            heap_up,
            heap_down,
            exporter,
        }
    }

    /// reseting its internal state
    fn reset_state(&mut self) {
        self.visited_up.invalidate_all();
        self.visited_down.invalidate_all();
        self.heap_up.clear();
        self.heap_down.clear();
        self.exporter.reset();
    }

    /// return shortest path of nodes
    fn find_path(
        &mut self,
        from: NodeId,
        to: NodeId,
        alpha: Vec<f64>,
        graph: &Graph,
        nodes: &[Node],
        mlp_layers: &[usize],
    ) -> Option<(Vec<NodeId>, Cost)> {
        self.reset_state();

        if from == to {
            return Some((vec![], 0.0));
        }

        self.visited_up.set_valid(from);
        self.dist_up[from] = (0.0, None);
        self.visited_down.set_valid(to);
        self.dist_down[to] = (0.0, None);

        self.heap_up.push(MinHeapItem::new(from, 0.0, None));
        self.heap_down.push(MinHeapItem::new(to, 0.0, None));

        let mut best_cost = COST_MAX;
        let mut meeting_node = None;

        // get maximum crp-layer partition
        // let highes_diff_layer =
        //     mlp_helper::get_highest_differing_level(from, node, &nodes, &mlp_layers);
        // let common_partition =
        //     mlp_helper::get_partition_id_on_level(from, highes_diff_layer, &nodes, &mlp_layers);
        // println!(
        //     "highes_diff_layer {:?} common_partition {:?}",
        //     highes_diff_layer, common_partition
        // );

        // function pointers for only having one single dijkstra
        let get_up_edge_ids: fn(&Graph, NodeId) -> Vec<EdgeId> = Graph::get_up_edge_ids;
        let get_down_edge_ids: fn(&Graph, NodeId) -> Vec<EdgeId> = Graph::get_down_edge_ids;
        let get_to: fn(&Edge) -> NodeId = Edge::get_to;
        let get_from: fn(&Edge) -> NodeId = Edge::get_from;

        while let Some((
            MinHeapItem {
                node,
                cost,
                prev_edge: _,
            },
            heap,
            visited,
            dist,
            walk,
            get_edges,
            visited_,
            dist_,
            exporter,
        )) = {
            let next_up: Cost = self
                .heap_up
                .peek()
                .map(|min_item| min_item.cost)
                .unwrap_or(COST_MAX);
            let next_down: Cost = self
                .heap_down
                .peek()
                .map(|min_item| min_item.cost)
                .unwrap_or(COST_MAX);
            if next_up + next_down > best_cost {
                None
            } else if next_up <= next_down {
                self.heap_up.pop().map(|x| {
                    (
                        x,
                        &mut self.heap_up,
                        &mut self.visited_up,
                        &mut self.dist_up,
                        get_to,
                        get_up_edge_ids,
                        &mut self.visited_down,
                        &mut self.dist_down,
                        &mut self.exporter,
                    )
                })
            } else {
                self.heap_down.pop().map(|x| {
                    (
                        x,
                        &mut self.heap_down,
                        &mut self.visited_down,
                        &mut self.dist_down,
                        get_from,
                        get_down_edge_ids,
                        &mut self.visited_up,
                        &mut self.dist_up,
                        &mut self.exporter,
                    )
                })
            }
        } {
            exporter.heap_pop();
            // node has already been visited and can be skipped
            // replacement for decrease key operation
            if visited.is_valid(node) && cost > dist[node].0 {
                continue;
            }

            // get query level on which to walk
            let query_layer = std::cmp::min(
                mlp_helper::get_highest_differing_level(from, node, &nodes, &mlp_layers),
                mlp_helper::get_highest_differing_level(node, to, &nodes, &mlp_layers),
            );

            for edge_id in get_edges(&graph, node) {
                let edge = graph.get_edge(edge_id);

                // skip edges, that are shortcuts-resultions from upper layers
                if !edge.core {
                    continue;
                }
                // only walk on query layers and never below
                if query_layer > edge.layer {
                    continue;
                }

                let next = walk(&edge);
                exporter.relaxed_edge();

                let alt = cost + costs_by_alpha(&graph.get_edge_costs(edge_id), &alpha);

                if !visited.is_valid(next) || alt < dist[next].0 {
                    heap.push(MinHeapItem::new(next, alt, Some(edge_id)));
                    visited.set_valid(next);
                    dist[next] = (alt, Some(edge_id));

                    exporter.visited_node(next);
                    exporter.visited_edge(Some(edge_id));
                    // check if other dijkstra has visited this point before
                    if visited_.is_valid(next) {
                        let combined = dist_[next].0 + alt;
                        if combined < best_cost {
                            meeting_node = Some(next);
                            exporter.current_meeting_point(next);
                            best_cost = combined;
                        }
                    }
                }
            }
        }
        match meeting_node {
            Some(meet_node) => {
                Some(self.resolve_path(meet_node, best_cost, nodes[meet_node].rank, &graph.edges))
            }
            None => None,
        }
    }
}

impl<E: Export> Dijkstra<E> {
    /// recreate path backwards
    fn resolve_path(
        &self,
        meeting_node: NodeId,
        cost: Cost,
        meeting_rank: Rank,
        edges: &[Edge],
    ) -> (Vec<NodeId>, Cost) {
        assert!(self.visited_up.is_valid(meeting_node));
        assert!(self.visited_down.is_valid(meeting_node));

        let mut path: Vec<NodeId> = Vec::with_capacity(meeting_rank.pow(2));

        let up_edge = self.dist_up[meeting_node];
        let down_edge = self.dist_down[meeting_node];

        if let Some(prev_edge) = up_edge.1 {
            self.walk_down(prev_edge, true, &mut path, &edges);
        }
        path.reverse();
        if let Some(prev_edge) = down_edge.1 {
            self.walk_down(prev_edge, false, &mut path, &edges);
        }
        (path, cost)
    }

    // walk shortcuts from meeting point to end
    fn walk_down(
        &self,
        edge: EdgeId,
        is_upwards: bool,
        mut path: &mut Vec<NodeId>,
        edges: &[Edge],
    ) {
        self.resolve_edge(edge, &mut path, is_upwards, &edges);

        let current_edge = &edges[edge];
        let prev;

        if is_upwards {
            prev = self.dist_up[current_edge.from];
        } else {
            prev = self.dist_down[current_edge.to];
        }
        if let Some(child) = prev.1 {
            self.walk_down(child, is_upwards, &mut path, &edges);
        }
    }

    /// resolve shortcuts to original edges
    fn resolve_edge(
        &self,
        edge: EdgeId,
        mut path: &mut Vec<NodeId>,
        is_upwards: bool,
        edges: &[Edge],
    ) {
        match &edges[edge].contrated_edges {
            Some(shortcut) => {
                if is_upwards {
                    self.resolve_edge(shortcut.1, &mut path, is_upwards, &edges);
                    self.resolve_edge(shortcut.0, &mut path, is_upwards, &edges);
                } else {
                    self.resolve_edge(shortcut.0, &mut path, is_upwards, &edges);
                    self.resolve_edge(shortcut.1, &mut path, is_upwards, &edges);
                }
            }
            None => path.push(edge),
        }
    }
}

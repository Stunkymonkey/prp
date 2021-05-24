// based on https://rosettacode.org/wiki/Dijkstra%27s_algorithm#Rust

use super::*;
use dijkstra_export::*;
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
    dim: usize,
    pub exporter: E,
}

impl<E: Export> Dijkstra<E> {
    /// general constructor
    pub fn new(amount_nodes: usize, dim: usize, exporter: E) -> Self {
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
            dim,
            exporter,
        }
    }

    /// reseting its internal state
    pub fn reset_state(&mut self) {
        self.visited_up.invalidate_all();
        self.visited_down.invalidate_all();
        self.heap_up.clear();
        self.heap_down.clear();
        self.exporter.reset();
    }

    /// return shortest path of nodes
    pub fn find_path(
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
        let mut meeting_node = INVALID_NODE;

        // get maximum crp-layer partition
        let highes_diff_layer =
            mlp_helper::get_highest_differing_level(from, to, &nodes, &mlp_layers);
        let common_partition =
            mlp_helper::get_partition_id_on_level(from, highes_diff_layer, &nodes, &mlp_layers);
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
                prev_edge,
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
            if self.heap_up.is_empty() && self.heap_down.is_empty() {
                return None;
            }
            let next_up: Cost = self
                .heap_up
                .peek()
                .unwrap_or(&MinHeapItem::new(INVALID_NODE, COST_MAX, None))
                .cost;
            let next_down: Cost = self
                .heap_down
                .peek()
                .unwrap_or(&MinHeapItem::new(INVALID_NODE, COST_MAX, None))
                .cost;
            // TODO maybe also look what node is on lower layer and pick that one
            if next_up + next_down >= best_cost {
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

            visited.set_valid(node);
            dist[node] = (cost, prev_edge);

            exporter.visited_node(node);
            exporter.visited_edge(prev_edge);

            for edge in get_edges(&graph, node) {
                let next = walk(&graph.get_edge(edge));

                exporter.relaxed_edge();

                // in lowest layer
                if nodes[node].layer_height == 0 {
                    // skip pch ranks
                    // top-layer nodes have maximum layer number so no equal test
                    if nodes[node].rank > nodes[next].rank {
                        break;
                    }
                // in top layer
                } else {
                    // walk only in layers above
                    if nodes[node].layer_height > nodes[next].layer_height {
                        break;
                    }
                    // do not walk in partitons, that are excluded
                    if mlp_helper::get_partition_id_on_level(
                        next,
                        highes_diff_layer,
                        &nodes,
                        &mlp_layers,
                    ) != common_partition
                    {
                        continue;
                    }
                    // do not walk in partitons higher then the common one
                    if graph.get_edge(edge).layer > highes_diff_layer {
                        continue;
                    }
                }

                let alt = cost + costs_by_alpha(&graph.get_edge_costs(edge), &alpha);

                if !visited.is_valid(next) || alt < dist[next].0 {
                    heap.push(MinHeapItem::new(next, alt, Some(edge)));

                    // check if other dijkstra has visited this point before
                    if visited_.is_valid(node) {
                        let combined = dist_[node].0 + alt;
                        if combined < best_cost {
                            meeting_node = node;
                            exporter.current_meeting_point(node);
                            best_cost = combined;
                        }
                    }
                }
            }
        }
        if meeting_node == INVALID_NODE {
            None
        } else {
            Some(self.resolve_path(
                meeting_node,
                best_cost,
                nodes[meeting_node].rank,
                &graph.edges,
            ))
        }
    }

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

        path.push(meeting_node);
        if let Some(prev_edge) = up_edge.1 {
            self.walk_down(prev_edge, true, &mut path, &edges);
            path.reverse();
        }
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
        let next;
        let prev;

        if is_upwards {
            next = current_edge.from;
            prev = self.dist_up[next];
        } else {
            next = current_edge.to;
            prev = self.dist_down[next];
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
        let current_edge = &edges[edge];

        if is_upwards {
            if let Some(next) = current_edge.contrated_edges {
                self.resolve_edge(next.1, &mut path, is_upwards, &edges);
            }
            if let Some(previous) = current_edge.contrated_edges {
                self.resolve_edge(previous.0, &mut path, is_upwards, &edges);
            } else {
                path.push(current_edge.from);
            }
        } else {
            if let Some(previous) = current_edge.contrated_edges {
                self.resolve_edge(previous.0, &mut path, is_upwards, &edges);
            }
            if let Some(next) = current_edge.contrated_edges {
                self.resolve_edge(next.1, &mut path, is_upwards, &edges);
            } else {
                path.push(current_edge.to);
            }
        }
    }
}

// based on https://rosettacode.org/wiki/Dijkstra%27s_algorithm#Rust

use super::*;
use mch::costs_by_alpha;
use min_heap::*;
use std::collections::BinaryHeap;
use valid_flag::*;

#[derive(Clone)]
pub struct Dijkstra {
    dist_up: Vec<(Cost, Option<NodeId>)>,
    dist_down: Vec<(Cost, Option<NodeId>)>,
    visited_up: ValidFlag,
    visited_down: ValidFlag,
    heap_up: BinaryHeap<MinHeapItem>,
    heap_down: BinaryHeap<MinHeapItem>,
    dim: usize,
}

impl Dijkstra {
    /// general constructor
    pub fn new(amount_nodes: usize, dim: usize) -> Self {
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
        }
    }

    /// reseting its internal state
    fn reset_state(&mut self) {
        self.visited_up.invalidate_all();
        self.visited_down.invalidate_all();
        self.heap_up.clear();
        self.heap_down.clear();
    }

    /// return shortest path of nodes
    pub fn find_path(
        &mut self,
        from: NodeId,
        to: NodeId,
        alpha: Vec<f64>,
        graph: &Graph,
        nodes: &[Node],
    ) -> Option<(Vec<NodeId>, Cost)> {
        self.reset_state();

        println!(
            "from {:?} [{:?}] \nto {:?} [{:?}]",
            from, nodes[from], to, nodes[to]
        );

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
        )) = {
            let next_up = self
                .heap_up
                .peek()
                .unwrap_or(&MinHeapItem::new(INVALID_NODE, COST_MAX, None))
                .cost;
            let next_down = self
                .heap_down
                .peek()
                .unwrap_or(&MinHeapItem::new(INVALID_NODE, COST_MAX, None))
                .cost;
            // println!(
            //     "next_up + next_down {:?} best_cost {:?}",
            //     next_up + next_down,
            //     best_cost
            // );
            if (next_up == COST_MAX && next_down == COST_MAX)
            // || (next_up + next_down).is_infinite()
            // || next_up + next_down >= best_cost
            {
                println!("breaking pop");
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
                    )
                })
            }
        } {
            // if visited.is_valid(node) && cost > dist[node].0 {
            //     continue;
            // }
            // visited.set_valid(node);
            // dist[node] = (cost, prev_edge);

            // for edge in get_edges(&graph, node) {
            //     let next = walk(&graph.get_edge(edge));

            //     // skip nodes with lower rank and lower layer_height
            //     if nodes[next].layer_height < nodes[node].layer_height
            //         || (nodes[next].layer_height == nodes[node].layer_height
            //             && nodes[next].rank <= nodes[node].rank)
            //     {
            //         break;
            //     }

            //     let alt = cost + costs_by_alpha(&graph.get_edge_costs(edge), &alpha);

            //     if !visited.is_valid(next) || (visited.is_valid(next) && alt < dist[next].0) {
            //         heap.push(MinHeapItem::new(next, alt, Some(edge)));
            //     }
            // }
            // if visited_.is_valid(node) && cost + dist_[node].0 < best_cost {
            //     best_cost = cost + dist_[node].0;
            //     meeting_node = node;
            // }
            if visited.is_valid(node) && cost == dist[node].0 {
                for edge in get_edges(&graph, node) {
                    let next = walk(&graph.get_edge(edge));

                    println!(
                        "node {:?} next {:?} node {:?} \t skip: {:?}",
                        node,
                        next,
                        nodes[next],
                        nodes[node].layer_height > nodes[next].layer_height
                    );

                    if nodes[node].layer_height > nodes[next].layer_height
                    // || (nodes[next].layer_height == nodes[node].layer_height
                    //     && nodes[next].rank <= nodes[node].rank)
                    {
                        // break;
                        continue;
                    }

                    let alt = cost + costs_by_alpha(&graph.get_edge_costs(edge), &alpha);
                    println!("visit {:?} new_cost {:?}", !visited.is_valid(next), alt);
                    if !visited.is_valid(next) || (visited.is_valid(next) && dist[next].0 > alt) {
                        dist[next] = (alt, prev_edge);
                        println!(
                            "pushing next {:?} \t alt {:?} \t  edge{:?}",
                            next, alt, edge
                        );
                        heap.push(MinHeapItem::new(next, alt, Some(edge)));

                        if visited_.is_valid(next) {
                            meeting_node = node;
                            best_cost = dist[next].0 + dist_[next].0;
                        }
                    }
                }
            }
        }
        if meeting_node == INVALID_NODE {
            None
        } else {
            println!("meeting {:?}", meeting_node);
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

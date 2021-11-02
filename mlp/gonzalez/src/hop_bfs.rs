use super::*;
use std::collections::{BTreeSet, VecDeque};
use valid_flag::*;

#[derive(Clone)]
pub struct HopBFS {
    dist: Vec<(Cost, Option<NodeId>)>,
    queue: VecDeque<(NodeId, Cost, Option<EdgeId>)>,
    visited: ValidFlag,
}

impl HopBFS {
    /// general constructor
    pub fn new(amount_nodes: usize) -> Self {
        let dist = vec![(COST_MAX, None); amount_nodes];
        let queue = VecDeque::with_capacity(amount_nodes / 4);
        let visited = ValidFlag::new(amount_nodes);
        HopBFS {
            dist,
            queue,
            visited,
        }
    }

    /// reseting its internal state
    pub fn reset_state(&mut self) {
        self.queue.clear();
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

        self.queue.push_front((from, 0, None));

        while let Some((node, dist, prev_edge)) = self.queue.pop_front() {
            // node has already been visited and can be skipped
            // replacement for decrease key operation
            if self.visited.is_valid(node) {
                continue;
            }

            self.visited.set_valid(node);
            self.dist[node] = (dist, prev_edge);

            if distances[node].0 > dist {
                distances[node] = (dist, from)
            }

            for edge in graph_helper::get_up_edge_ids(node, offset) {
                let new_edge = &edges[edge];

                // skip edges, that go outside of the parent-set
                if !current_set.contains(&new_edge.to) {
                    continue;
                }
                // skip edges, that are further away then current distance
                if distances[new_edge.to].0 < dist {
                    continue;
                }

                let alt = dist + 1;
                if !self.visited.is_valid(new_edge.to) {
                    self.queue.push_back((new_edge.to, alt, Some(edge)));
                }
            }
        }
    }
}

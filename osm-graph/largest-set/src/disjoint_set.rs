use super::*;

use std::collections::{BTreeMap, BTreeSet};

pub fn get_largest_disjoint_set(nodes: &[Node], edges: &[Edge]) -> Vec<NodeId> {
    // create offsets
    let mut up_offset = Vec::<EdgeId>::new();
    up_offset.resize(nodes.len() + 1, 0);
    let sources: Vec<EdgeId> = edges.par_iter().map(|x| x.from).rev().collect();
    offset::fill_offset(sources, &mut up_offset);

    // to store best set
    let mut visited = vec![false; nodes.len()];
    let mut largest_set: BTreeSet<NodeId> = BTreeSet::new();
    let mut queue: Vec<NodeId> = Vec::with_capacity((nodes.len() as f64).ln() as usize);

    // get largest
    for node_id in 0..nodes.len() {
        if !visited[node_id] {
            // create empty collection
            let mut tmp = BTreeSet::new();
            queue.push(node_id);

            while let Some(next_node) = queue.pop() {
                // skip if already visited
                if visited[next_node] {
                    continue;
                }
                // visit
                visited[next_node] = true;
                tmp.insert(next_node);

                // add not visited neigbors to queue
                for edge in edges
                    .iter()
                    .take(up_offset[next_node + 1])
                    .skip(up_offset[next_node])
                {
                    if !visited[edge.to] {
                        queue.push(edge.to);
                    }
                }
            }
            // only save largest
            if tmp.len() > largest_set.len() {
                largest_set = tmp;
            }
        }
    }
    // collect and sort them, for easier index work later
    let mut result: Vec<NodeId> = largest_set.into_iter().collect();
    result.par_sort_unstable();
    result
}

pub fn keep_only_nodes_from_set(
    keeping_nodes: &[NodeId],
    nodes: &[Node],
    edges: &[Edge],
) -> (Vec<Node>, Vec<Edge>) {
    // get new ids
    let mut new_node_ids = BTreeMap::new();
    for (new_node_id, old_node_id) in keeping_nodes.iter().enumerate() {
        new_node_ids.insert(old_node_id, new_node_id);
    }

    // keep wanted edges and change indices
    let mut resulting_edges = Vec::with_capacity(edges.len());
    for edge in edges {
        if new_node_ids.contains_key(&edge.from) {
            let from = *new_node_ids.get(&edge.from).unwrap();
            let to = *new_node_ids.get(&edge.to).unwrap();
            resulting_edges.push(Edge::new(
                from,
                to,
                edge.cost.clone(),
                edge.contracted_edges,
            ));
        }
    }

    // only store wanted nodes
    let mut resulting_nodes = Vec::with_capacity(nodes.len());
    for node_id in keeping_nodes {
        resulting_nodes.push(nodes[*node_id].clone());
    }

    (resulting_nodes, resulting_edges)
}

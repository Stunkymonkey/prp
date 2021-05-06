use super::*;

use std::collections::BTreeSet;

/// find single set with depth search
fn depth_search(
    set: &mut BTreeSet<NodeId>,
    node_id: NodeId,
    edges: &[Edge],
    up_offset: &[EdgeId],
    visited: &mut [bool],
) {
    visited[node_id] = true;
    set.insert(node_id);

    for edge_id in up_offset[node_id]..up_offset[node_id + 1] {
        let next_node = edges[edge_id].to;
        if !visited[next_node] {
            depth_search(set, next_node, &edges, &up_offset, visited);
        }
    }
}

/// find largest disjoint set
pub fn get_disjoint_nodes(nodes: &[Node], edges: &[Edge]) -> Vec<NodeId> {
    // create offsets
    let mut up_offset = Vec::<EdgeId>::new();
    up_offset.resize(nodes.len() + 1, 0);
    let sources: Vec<EdgeId> = edges.par_iter().map(|x| x.from).rev().collect();
    offset::fill_offset(sources, &mut up_offset);

    let mut visited = vec![false; nodes.len()];

    let mut largest_set: BTreeSet<NodeId> = BTreeSet::new();

    // get largest
    for node_id in 0..nodes.len() {
        if !visited[node_id] {
            let mut tmp = BTreeSet::new();
            depth_search(&mut tmp, node_id, &edges, &up_offset, &mut visited);
            if tmp.len() > largest_set.len() {
                largest_set = tmp;
            }
        }
    }
    let mut result: Vec<NodeId> = largest_set.into_iter().collect();
    result.par_sort_unstable();
    result
}

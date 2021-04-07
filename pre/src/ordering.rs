use super::*;
use valid_flag::ValidFlag;

use std::collections::BTreeSet;

/// heuristic by neighbors
#[allow(dead_code)]
pub fn calculate_single_heuristic(
    node: NodeId,
    up_offset: &[EdgeId],
    down_offset: &[EdgeId],
) -> usize {
    up_offset[node + 1] - up_offset[node] * down_offset[node + 1] - down_offset[node]
}

/// calculate heuristic in parallel
#[allow(clippy::too_many_arguments)]
pub fn calculate_heuristics(
    amount_nodes: usize,
    up_offset: &[EdgeId],
    down_offset: &[EdgeId],
) -> Vec<usize> {
    let mut heuristics: Vec<usize> = Vec::with_capacity(amount_nodes);
    for node_id in 0..amount_nodes {
        heuristics.push(calculate_single_heuristic(
            node_id,
            &up_offset,
            &down_offset,
        ));
    }
    heuristics
}

/// update all direct neighbors
#[allow(clippy::too_many_arguments)]
pub fn update_neighbor_heuristics(
    neighbors: Vec<NodeId>,
    heuristics: &mut Vec<usize>,
    up_offset: &[EdgeId],
    down_offset: &[EdgeId],
) {
    for neighbor in neighbors {
        heuristics[neighbor] = calculate_single_heuristic(neighbor, &up_offset, &down_offset);
    }
}

/// get independent set of graph using heuristic
pub fn get_independent_set(
    remaining_nodes: &BTreeSet<NodeId>,
    heuristics: &[usize],
    minimas_bool: &mut ValidFlag,
    edges: &[Edge],
    up_offset: &[EdgeId],
    down_offset: &[EdgeId],
    down_index: &[NodeId],
) -> Vec<NodeId> {
    let subset: Vec<NodeId>;
    let mut remaining_nodes_vector: Vec<NodeId> = remaining_nodes.iter().copied().collect();
    if remaining_nodes.len() > 10_000 {
        // sort remaining_nodes via heuristic
        remaining_nodes_vector.par_sort_by_key(|&node| heuristics[node]);
        // take lower 1/4
        // TODO maybe do this adaptive
        subset = (&remaining_nodes_vector[0..remaining_nodes_vector.len() / 4]).to_vec();
    } else {
        subset = remaining_nodes_vector;
    }

    minimas_bool.invalidate_all();
    // mark all neighbors with greater equal value as invalid
    for node in &subset {
        for neighbor in
            graph_helper::get_all_neighbours(*node, &edges, &up_offset, &down_offset, &down_index)
        {
            if !minimas_bool.is_valid(neighbor)
                && neighbor != *node
                && heuristics[*node] >= heuristics[neighbor]
            {
                minimas_bool.set_valid(*node);
            }
        }
    }

    // collect all indices of valid nodes
    let result: Vec<NodeId> = subset
        .par_iter()
        .filter(|&node| !minimas_bool.is_valid(*node))
        .map(|node| *node)
        .collect();
    result
}

#[test]
fn independent_set_test() {
    // note: in this test no edge gets removed
    // 0->1->2->3->4->5->6->7->8
    let amount_nodes = 9;

    let mut remaining_nodes = BTreeSet::new();
    for node_id in 0..amount_nodes {
        remaining_nodes.insert(node_id);
    }

    let mut edges = Vec::<Edge>::new();
    edges.push(Edge::new(0, 1, vec![1.0]));
    edges.push(Edge::new(1, 2, vec![1.0]));
    edges.push(Edge::new(2, 3, vec![1.0]));
    edges.push(Edge::new(3, 4, vec![1.0]));
    edges.push(Edge::new(4, 5, vec![1.0]));
    edges.push(Edge::new(5, 6, vec![1.0]));
    edges.push(Edge::new(6, 7, vec![1.0]));
    edges.push(Edge::new(7, 8, vec![1.0]));

    let mut up_offset = Vec::<EdgeId>::new();
    let mut down_offset = Vec::<EdgeId>::new();
    let down_index =
        offset::generate_offsets(&mut edges, &mut up_offset, &mut down_offset, amount_nodes);

    let heuristics = vec![2, 3, 0, 3, 6, 5, 3, 1, 7];

    let mut minimas_bool = ValidFlag::new(amount_nodes);

    let minima = get_independent_set(
        &remaining_nodes,
        &heuristics,
        &mut minimas_bool,
        &edges,
        &up_offset,
        &down_offset,
        &down_index,
    );

    let expected_minima = vec![0, 2, 7];

    assert_eq!(minima, expected_minima);

    remaining_nodes.remove(&0);
    remaining_nodes.remove(&2);
    remaining_nodes.remove(&7);

    let heuristics = vec![99, 1, 99, 1, 4, 3, 1, 99, 5];
    let minima = get_independent_set(
        &remaining_nodes,
        &heuristics,
        &mut minimas_bool,
        &edges,
        &up_offset,
        &down_offset,
        &down_index,
    );
    let expected_minima = vec![1, 3, 6, 8];

    assert_eq!(minima, expected_minima);
}

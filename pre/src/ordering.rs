use super::*;
use valid_flag::ValidFlag;

use std::collections::BTreeSet;

/// heuristic by neighbors
#[allow(dead_code)]
pub fn calculate_single_heuristic(
    node: NodeId,
    _deleted_neighbors: &[NodeId],
    up_offset: &[EdgeId],
    down_offset: &[EdgeId],
) -> usize {
    (up_offset[node + 1] - up_offset[node]) * (down_offset[node + 1] - down_offset[node])
    // + _deleted_neighbors[node]
}

/// calculate heuristic in parallel
#[allow(clippy::too_many_arguments)]
pub fn calculate_heuristics(
    level_height: Level,
    nodes: &[Node],
    deleted_neighbors: &[NodeId],
    up_offset: &[EdgeId],
    down_offset: &[EdgeId],
) -> Vec<usize> {
    let mut heuristics: Vec<usize> = Vec::with_capacity(nodes.len());
    for (node_id, node) in nodes.iter().enumerate() {
        if node.level != level_height {
            heuristics.push(usize::MAX);
        } else {
            heuristics.push(calculate_single_heuristic(
                node_id,
                deleted_neighbors,
                up_offset,
                down_offset,
            ));
        }
    }
    heuristics
}

/// update all direct neighbors
#[allow(clippy::too_many_arguments)]
pub fn update_neighbor_heuristics(
    neighbors: Vec<NodeId>,
    level_height: Level,
    heuristics: &mut [usize],
    nodes: &[Node],
    deleted_neighbors: &[NodeId],
    up_offset: &[EdgeId],
    down_offset: &[EdgeId],
) {
    for neighbor in neighbors {
        if nodes[neighbor].level == level_height {
            heuristics[neighbor] =
                calculate_single_heuristic(neighbor, deleted_neighbors, up_offset, down_offset);
        }
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
    let mut remaining_nodes_vector: Vec<NodeId> = remaining_nodes.iter().copied().collect();
    let subset: Vec<NodeId> = if remaining_nodes.len() > 10 {
        // sort remaining_nodes via heuristic
        remaining_nodes_vector.par_sort_unstable_by_key(|&node| heuristics[node]);

        // take lower 1/10 and round up by adding the divider minus one
        remaining_nodes_vector[0..((remaining_nodes_vector.len() + 10 - 1) / 10)].to_vec()
    } else {
        remaining_nodes_vector
    };

    minimas_bool.invalidate_all();
    // mark all neighbors with greater equal value as invalid
    for node in &subset {
        let mut is_valid = true;
        for neighbor in
            graph_helper::get_all_neighbours(*node, edges, up_offset, down_offset, down_index)
        {
            if minimas_bool.is_valid(neighbor)
                || *node == neighbor
                || heuristics[*node] > heuristics[neighbor]
            {
                is_valid = false;
                break;
            }
        }
        if is_valid {
            minimas_bool.set_valid(*node);
        }
    }

    // collect all indices of valid nodes
    subset
        .par_iter()
        .filter(|&node| minimas_bool.is_valid(*node))
        .map(|node| *node)
        .collect()
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

    let mut edges = vec![
        Edge::new(0, 1, vec![1.0]),
        Edge::new(1, 2, vec![1.0]),
        Edge::new(2, 3, vec![1.0]),
        Edge::new(3, 4, vec![1.0]),
        Edge::new(4, 5, vec![1.0]),
        Edge::new(5, 6, vec![1.0]),
        Edge::new(6, 7, vec![1.0]),
        Edge::new(7, 8, vec![1.0]),
    ];

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

use super::*;

/// get in what PartitionId it is on a specific level
pub fn get_partition_id_on_level(
    node_id: NodeId,
    level: usize,
    nodes: &[Node],
    mlp_levels: &[usize],
) -> PartitionId {
    let partition = nodes[node_id].partition;
    if level == 0 {
        partition
    } else {
        partition / mlp_levels.iter().take(level).product::<usize>()
    }
}

// get all partiton_ids of one node (including top level)
pub fn get_node_partitions(
    node_id: NodeId,
    nodes: &[Node],
    mlp_levels: &[usize],
) -> Vec<PartitionId> {
    let mut partitions = Vec::with_capacity(mlp_levels.len() + 1);
    for level in 0..=mlp_levels.len() {
        partitions.push(get_partition_id_on_level(node_id, level, nodes, mlp_levels));
    }
    partitions
}

// find most common level height (0 = same partition, ...)
pub fn get_highest_differing_level_partition(
    node_a: NodeId,
    partitions_b: &[PartitionId],
    nodes: &[Node],
    mlp_levels: &[usize],
) -> usize {
    assert_eq!(mlp_levels.len() + 1, partitions_b.len());
    for (level, partition) in partitions_b.iter().enumerate().take(mlp_levels.len() + 1) {
        if get_partition_id_on_level(node_a, level, nodes, mlp_levels) == *partition {
            return level;
        }
    }
    panic!("no common level found")
}

// find most common level height (0 = same partition, ...)
pub fn get_highest_differing_level(
    node_a: NodeId,
    node_b: NodeId,
    nodes: &[Node],
    mlp_levels: &[usize],
) -> usize {
    for level in 0..=mlp_levels.len() {
        if get_partition_id_on_level(node_a, level, nodes, mlp_levels)
            == get_partition_id_on_level(node_b, level, nodes, mlp_levels)
        {
            return level;
        }
    }
    panic!("no common level found")
}

// convert the partition_id to the level
pub fn calculate_node_levels(
    node_id: NodeId,
    nodes: &[Node],
    graph: &Graph,
    mlp_levels: &[usize],
) -> usize {
    graph
        .get_all_edge_ids(node_id)
        .iter()
        .map(|edge_id| {
            let edge = graph.get_edge(*edge_id);
            // only calculated via edges, that existed before contraction
            if edge.contracted_edges.is_some() {
                0
            } else {
                get_highest_differing_level(edge.from, edge.to, nodes, mlp_levels)
            }
        })
        .max()
        .unwrap_or(0)
}

// convert the partition_ids to the level_height
pub fn calculate_levels(nodes: &[Node], graph: &Graph, mlp_levels: &[usize]) -> Vec<usize> {
    // only calculated via edges, that existed before contraction
    let highest_edge_diff: Vec<_> = graph
        .edges
        .iter()
        .map(|edge| {
            if edge.contracted_edges.is_some() {
                0
            } else {
                get_highest_differing_level(edge.from, edge.to, nodes, mlp_levels)
            }
        })
        .collect();

    let mut level_height = Vec::with_capacity(nodes.len());
    for (node_id, _node) in nodes.iter().enumerate() {
        let node_edges = graph.get_all_edge_ids(node_id);
        level_height.push(
            node_edges
                .iter()
                .map(|edge_id| highest_edge_diff[*edge_id])
                .max()
                .unwrap_or(0),
        );
    }
    level_height
}

#[test]
fn level_partition_ids() {
    let partitions = vec![4, 3, 3];

    let mut nodes = Vec::<Node>::new();
    for partition in 0..36 {
        nodes.push(Node {
            latitude: 0.0,
            longitude: 0.0,
            rank: 0,
            partition,
        });
    }
    nodes.push(Node {
        latitude: 0.0,
        longitude: 0.0,
        rank: 0,
        partition: 27,
    });

    assert_eq!(get_partition_id_on_level(7, 0, &nodes, &partitions), 7);
    assert_eq!(get_partition_id_on_level(7, 1, &nodes, &partitions), 1);
    assert_eq!(get_partition_id_on_level(7, 2, &nodes, &partitions), 0);
    assert_eq!(get_partition_id_on_level(17, 0, &nodes, &partitions), 17);
    assert_eq!(get_partition_id_on_level(17, 1, &nodes, &partitions), 4);
    assert_eq!(get_partition_id_on_level(17, 2, &nodes, &partitions), 1);
    assert_eq!(get_partition_id_on_level(35, 0, &nodes, &partitions), 35);
    assert_eq!(get_partition_id_on_level(35, 1, &nodes, &partitions), 8);
    assert_eq!(get_partition_id_on_level(35, 2, &nodes, &partitions), 2);

    assert_eq!(get_highest_differing_level(27, 36, &nodes, &partitions), 0);
    assert_eq!(get_highest_differing_level(26, 36, &nodes, &partitions), 1);
    assert_eq!(get_highest_differing_level(26, 35, &nodes, &partitions), 2);
    assert_eq!(get_highest_differing_level(17, 29, &nodes, &partitions), 3);
    assert_eq!(get_highest_differing_level(0, 35, &nodes, &partitions), 3);
}

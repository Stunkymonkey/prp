use super::*;

/// get in what PartitionId it is on a specific layer
pub fn get_partition_id_on_level(
    node_id: NodeId,
    layer: usize,
    nodes: &[Node],
    mlp_layers: &[usize],
) -> PartitionId {
    let divisor = mlp_layers.iter().take(layer).product::<usize>();
    let partition = nodes[node_id].partition;
    if divisor == 0 {
        partition
    } else {
        partition / divisor
    }
}

// get all partiton_ids of one node (including top layer)
pub fn get_node_partitions(
    node_id: NodeId,
    nodes: &[Node],
    mlp_layers: &[usize],
) -> Vec<PartitionId> {
    let mut partitions = Vec::with_capacity(mlp_layers.len() + 1);
    for layer in 0..=mlp_layers.len() {
        partitions.push(get_partition_id_on_level(
            node_id,
            layer,
            &nodes,
            &mlp_layers,
        ));
    }
    partitions
}

// find most common layer height (0 = same partition, ...)
pub fn get_highest_differing_level_partition(
    node_a: NodeId,
    partitions_b: &[PartitionId],
    nodes: &[Node],
    mlp_layers: &[usize],
) -> usize {
    assert_eq!(mlp_layers.len() + 1, partitions_b.len());
    for (layer, partition) in partitions_b.iter().enumerate().take(mlp_layers.len() + 1) {
        if get_partition_id_on_level(node_a, layer, &nodes, &mlp_layers) == *partition {
            return layer;
        }
    }
    panic!("no common layer found")
}

// find most common layer height (0 = same partition, ...)
pub fn get_highest_differing_level(
    node_a: NodeId,
    node_b: NodeId,
    nodes: &[Node],
    mlp_layers: &[usize],
) -> usize {
    for layer in 0..=mlp_layers.len() {
        if get_partition_id_on_level(node_a, layer, &nodes, &mlp_layers)
            == get_partition_id_on_level(node_b, layer, &nodes, &mlp_layers)
        {
            return layer;
        }
    }
    panic!("no common layer found")
}

// convert the partition_id to the layer_height
pub fn calculate_node_layer_height(
    node_id: NodeId,
    nodes: &[Node],
    graph: &Graph,
    mlp_layers: &[usize],
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
                get_highest_differing_level(edge.from, edge.to, &nodes, &mlp_layers)
            }
        })
        .max()
        .unwrap_or(0)
}

// convert the partition_ids to the layer_height
pub fn calculate_node_layer_heights(
    nodes: &[Node],
    graph: &Graph,
    mlp_layers: &[usize],
) -> Vec<usize> {
    // only calculated via edges, that existed before contraction
    let highest_edge_diff: Vec<_> = graph
        .edges
        .iter()
        .map(|edge| {
            if edge.contracted_edges.is_some() {
                0
            } else {
                get_highest_differing_level(edge.from, edge.to, &nodes, &mlp_layers)
            }
        })
        .collect();

    let mut layer_height = Vec::with_capacity(nodes.len());
    for (node_id, _node) in nodes.iter().enumerate() {
        let node_edges = graph.get_all_edge_ids(node_id);
        layer_height.push(
            node_edges
                .iter()
                .map(|edge_id| highest_edge_diff[*edge_id])
                .max()
                .unwrap_or(0),
        );
    }
    layer_height
}

#[test]
fn layer_partition_ids() {
    let partitions = vec![4, 3, 3];

    let mut nodes = Vec::<Node>::new();
    for partition in 0..36 {
        nodes.push(Node {
            latitude: 0.0,
            longitude: 0.0,
            rank: 0,
            partition: partition,
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

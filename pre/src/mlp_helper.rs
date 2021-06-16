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
pub fn calculate_node_layer_heights(
    nodes: &mut [Node],
    edges: &[Edge],
    up_offset: &[EdgeId],
    down_offset: &[EdgeId],
    down_index: &[EdgeId],
    mlp_layers: &[usize],
) {
    // only calculated via edges, that existed before contraction
    let highest_edge_diff: Vec<_> = edges
        .iter()
        .filter(|edge| edge.contracted_edges.is_none())
        .map(|edge| {
            if edge.contracted_edges.is_some() {
                0
            } else {
                get_highest_differing_level(edge.from, edge.to, &nodes, &mlp_layers)
            }
        })
        .collect();

    for (node_id, mut node) in nodes.iter_mut().enumerate() {
        let node_edges =
            graph_helper::get_all_edge_ids(node_id, &up_offset, &down_offset, &down_index);
        node.layer_height = node_edges
            .iter()
            .map(|edge_id| highest_edge_diff[*edge_id])
            .max()
            .unwrap_or(0);
    }
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
            layer_height: INVALID_LAYER_HEIGHT,
            old_id: None,
        });
    }
    nodes.push(Node {
        latitude: 0.0,
        longitude: 0.0,
        rank: 0,
        partition: 27,
        layer_height: INVALID_LAYER_HEIGHT,
        old_id: None,
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

#[test]
fn node_height() {
    // 0-->1-->2-->3-->4-->5<--12
    // |       |            |
    // V       V            V
    // 6-->7-->8-->9-->10-->11-->13

    let mlp_layers = vec![3, 2];

    let mut nodes = Vec::<Node>::new();
    for partition in 0..12 {
        nodes.push(Node {
            latitude: 0.0,
            longitude: 0.0,
            rank: 0,
            partition: partition % 6,
            layer_height: INVALID_LAYER_HEIGHT,
            old_id: None,
        });
    }
    nodes.push(Node {
        latitude: 0.0,
        longitude: 0.0,
        rank: 0,
        partition: 5,
        layer_height: INVALID_LAYER_HEIGHT,
        old_id: None,
    });

    nodes.push(Node {
        latitude: 0.0,
        longitude: 0.0,
        rank: 0,
        partition: 5,
        layer_height: INVALID_LAYER_HEIGHT,
        old_id: None,
    });

    let mut edges = Vec::<Edge>::new();
    edges.push(Edge::new(0, 1, vec![1.0]));
    edges.push(Edge::new(0, 6, vec![1.0]));
    edges.push(Edge::new(1, 2, vec![1.0]));
    edges.push(Edge::new(2, 3, vec![1.0]));
    edges.push(Edge::new(3, 4, vec![1.0]));
    edges.push(Edge::new(4, 5, vec![1.0]));
    edges.push(Edge::new(5, 11, vec![1.0]));
    edges.push(Edge::new(6, 7, vec![1.0]));
    edges.push(Edge::new(7, 8, vec![1.0]));
    edges.push(Edge::new(8, 9, vec![1.0]));
    edges.push(Edge::new(9, 10, vec![1.0]));
    edges.push(Edge::new(10, 11, vec![1.0]));
    edges.push(Edge::new(11, 13, vec![1.0]));
    edges.push(Edge::new(12, 5, vec![1.0]));

    let mut up_offset = Vec::<EdgeId>::new();
    let mut down_offset = Vec::<EdgeId>::new();
    let down_index =
        offset::generate_offsets(&mut edges, &mut up_offset, &mut down_offset, nodes.len());

    calculate_node_layer_heights(
        &mut nodes,
        &edges,
        &up_offset,
        &down_offset,
        &down_index,
        &mlp_layers,
    );

    assert_eq!(nodes[0].layer_height, 1);
    assert_eq!(nodes[1].layer_height, 1);
    assert_eq!(nodes[2].layer_height, 2);
    assert_eq!(nodes[3].layer_height, 2);
    assert_eq!(nodes[4].layer_height, 1);
    assert_eq!(nodes[5].layer_height, 1);
    assert_eq!(nodes[6].layer_height, 1);
    assert_eq!(nodes[7].layer_height, 1);
    assert_eq!(nodes[8].layer_height, 2);
    assert_eq!(nodes[9].layer_height, 2);
    assert_eq!(nodes[10].layer_height, 1);
    assert_eq!(nodes[11].layer_height, 1);
    assert_eq!(nodes[12].layer_height, 0);
    assert_eq!(nodes[13].layer_height, 0);
}

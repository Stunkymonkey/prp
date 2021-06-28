use super::*;
use hop_dijkstra::HopDijkstra;
use rand::seq::SliceRandom;
use rand::{rngs::StdRng, SeedableRng};
use std::collections::BTreeSet;

pub fn partition(
    partition_amount: &[usize],
    nodes: &mut [structs::Node],
    edges: &mut Vec<Edge>,
) -> Result<(), String> {
    let mut partition_order: Vec<NodeId> = (0..nodes.len()).collect();
    let mut partition_indices: Vec<usize> = vec![0, nodes.len()];
    let mut rng: Box<StdRng> = Box::new(SeedableRng::seed_from_u64(42));

    let mut up_offset = Vec::<EdgeId>::new();
    up_offset.resize(nodes.len() + 1, 0);
    let sources: Vec<EdgeId> = edges.iter().map(|x| x.from).rev().collect();
    offset::fill_offset(sources, &mut up_offset);

    for cluster in partition_amount {
        let mut current_indices: Vec<usize> = vec![0];
        for (start, end) in partition_indices
            .iter()
            .zip(partition_indices.iter().skip(1))
        {
            // get nodes
            let current_cluster = &partition_order[*start..*end];
            // do clustering using hop-distance
            println!("partition {:?} size {:?}", cluster, end - start);

            let (mut new_indices, new_clusters) = match make_partition(
                *cluster,
                &current_cluster,
                &nodes,
                &edges,
                &up_offset,
                &mut rng,
            ) {
                Ok(result) => result,
                Err(error) => {
                    return Err(format!(
                        "partitioning did not work succesfully: {:?}",
                        error
                    ))
                }
            };
            // store new order of nodes
            for (index, &cluster) in (*start..*end).zip(new_clusters.iter()) {
                partition_order[index] = cluster;
            }
            // concat indices
            new_indices.iter_mut().for_each(|i| *i += start);
            current_indices.extend(&new_indices);
        }
        partition_indices = current_indices;
    }

    // resolve partitions from clusters
    for (i, (start, end)) in partition_indices
        .iter()
        .zip(partition_indices.iter().skip(1))
        .enumerate()
    {
        // assign same partition for nodes in one cluster
        for index in *start..*end {
            nodes[partition_order[index]].partition = i;
        }
    }
    Ok(())
}

fn make_partition(
    k: usize,
    node_ids: &[NodeId],
    nodes: &[Node],
    edges: &[Edge],
    up_offset: &[EdgeId],
    rng: &mut Box<StdRng>,
) -> Result<(Vec<usize>, Vec<NodeId>), usize> {
    let current_set: BTreeSet<NodeId> = node_ids.iter().cloned().collect();
    let mut distances: Vec<(Cost, NodeId)> = vec![(COST_MAX, INVALID_NODE); nodes.len()];
    let mut tmp: Vec<(Cost, NodeId)> = vec![(COST_MAX, INVALID_NODE); nodes.len()];

    // get random start
    let start = node_ids.choose(rng).unwrap();
    let mut dijkstra = HopDijkstra::new(nodes.len());

    let mut core_nodes: Vec<NodeId> = Vec::with_capacity(k + 1);

    // find most distant node to set as real start
    dijkstra.get_costs(*start, &edges, &up_offset, &current_set, &mut tmp);
    let mut start: usize = *node_ids
        .iter()
        .max_by(|a, b| (tmp[**a].0).cmp(&tmp[**b].0))
        .unwrap_or(&0);
    core_nodes.push(start);

    // run dijkstra for every partition again to find a good partitioning
    for _i in 0..k {
        dijkstra.get_costs(start, &edges, &up_offset, &current_set, &mut distances);
        start = *node_ids
            .iter()
            .max_by(|a, b| (distances[**a].0).cmp(&distances[**b].0))
            .unwrap_or(&0);
        core_nodes.push(start);
    }
    core_nodes.pop();

    let mut order = Vec::with_capacity(node_ids.len());
    let mut offsets = Vec::with_capacity(k);

    // generate offsets
    for core_node in &core_nodes {
        for node_id in node_ids {
            if distances[*node_id].1 == *core_node {
                order.push(*node_id);
            }
        }
        // save end index as end of cluster
        offsets.push(order.len());
    }

    Ok((offsets, order))
}

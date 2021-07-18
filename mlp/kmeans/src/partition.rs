use ndarray::Array2;

use crate::constants::*;
use crate::structs::*;

pub fn partition(partitions: &[usize], nodes: &mut Vec<Node>) -> Result<(), String> {
    // ids of all nodes
    let mut partition_order: Vec<NodeId> = (0..nodes.len()).collect();
    // indices which divides the partitions (including all at the beginning)
    let mut partition_indices: Vec<usize> = vec![0, nodes.len()];

    //(top to bottom iterrieren)
    for partition in partitions {
        let mut current_indices: Vec<usize> = vec![0];
        for (start, end) in partition_indices
            .iter()
            .zip(partition_indices.iter().skip(1))
        {
            // get nodes
            let current_partition = &partition_order[*start..*end];
            // do partitioning using kmeans/...
            println!("partition {:?} size {:?}", partition, end - start);
            // let (mut new_indices, new_partitions) = make_partition(*partition, &current_partition, &nodes);

            let (mut new_indices, new_partition) =
                match make_partition(*partition, &current_partition, &nodes) {
                    Ok(result) => result,
                    Err(error) => {
                        return Err(format!(
                            "partitioning did not converge after rounds: {:?}",
                            error
                        ))
                    }
                };
            // store new order of nodes
            for (index, &partition) in (*start..*end).zip(new_partition.iter()) {
                partition_order[index] = partition;
            }
            // concat indices
            new_indices.iter_mut().for_each(|i| *i += start);
            current_indices.extend(&new_indices);
        }
        partition_indices = current_indices;
    }

    // resolve partitions from partitions
    for (i, (start, end)) in partition_indices
        .iter()
        .zip(partition_indices.iter().skip(1))
        .enumerate()
    {
        // assign same partition for nodes in one partition
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
) -> Result<(Vec<usize>, Vec<NodeId>), usize> {
    let mut data = Vec::with_capacity(node_ids.len());
    for node in node_ids {
        data.push(nodes[*node].latitude);
        data.push(nodes[*node].longitude);
    }

    let ndata: Array2<f64> = Array2::from_shape_vec((data.len() / 2, 2), data).unwrap();
    let config = rkm::Config::from(Some((42_u128).to_le_bytes()), Some(10_000), None);
    let (_means, clusters) = rkm::kmeans_lloyd_with_config(&ndata.view(), k, &config);

    let mut order = Vec::with_capacity(node_ids.len());
    let mut offsets = Vec::with_capacity(k);

    // convert kmeans-id back to pbfextractor-id
    for cluster in 0..k {
        for (id, node_id) in node_ids.iter().enumerate() {
            if clusters[id] == cluster {
                order.push(*node_id);
            }
        }
        // save end index as end of cluster
        offsets.push(order.len());
    }

    Ok((offsets, order))
}

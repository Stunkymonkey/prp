use cogset::{Euclid, KmeansBuilder};

use crate::constants::*;
use crate::structs::*;

pub fn partition(clusters: &[usize], nodes: &mut Vec<Node>) -> Result<(), String> {
    // ids of all nodes
    let mut cluster_order: Vec<NodeId> = (0..nodes.len()).collect();
    // indices which divides the clusters (including all at the beginning)
    let mut cluster_indices: Vec<usize> = Vec::new();
    cluster_indices.push(0);
    cluster_indices.push(nodes.len());

    //(top to bottom iterrieren)
    for cluster in clusters {
        let mut current_indices: Vec<usize> = Vec::new();
        current_indices.push(0);
        for (start, end) in cluster_indices.iter().zip(cluster_indices.iter().skip(1)) {
            // get nodes
            let current_cluster = &cluster_order[*start..*end];
            // do clustering using kmeans/...
            println!("cluster {:?} size {:?}", cluster, end - start);
            // let (mut new_indices, new_clusters) = make_cluster(*cluster, &current_cluster, &nodes);

            let (mut new_indices, new_clusters) =
                match make_cluster(*cluster, &current_cluster, &nodes) {
                    Ok(result) => result,
                    Err(error) => {
                        return Err(format!(
                            "clustering did not converge after rounds: {:?}",
                            error
                        ))
                    }
                };
            // store new order of nodes
            for (index, &cluster) in (*start..*end).zip(new_clusters.iter()) {
                cluster_order[index] = cluster;
            }
            // concat indices
            new_indices.iter_mut().for_each(|i| *i += start);
            current_indices.extend(&new_indices);
        }
        cluster_indices = current_indices;
    }

    // resolve partitions from clusters
    for (i, (start, end)) in cluster_indices
        .iter()
        .zip(cluster_indices.iter().skip(1))
        .enumerate()
    {
        // assign same partition for nodes in one cluster
        for index in *start..*end {
            nodes[cluster_order[index]].cluster = i;
        }
    }
    Ok(())
}

fn make_cluster(
    k: usize,
    node_ids: &[NodeId],
    nodes: &[Node],
) -> Result<(Vec<usize>, Vec<NodeId>), usize> {
    let mut data = Vec::with_capacity(node_ids.len());
    for node in node_ids {
        data.push(Euclid([nodes[*node].latitude, nodes[*node].longitude]))
    }

    // TODO: check; maybe this needs to be higher
    let kmeans = KmeansBuilder::new().max_iter(10_000).kmeans(&data, k);

    kmeans.converged()?;

    let mut order = Vec::with_capacity(node_ids.len());
    let mut offsets = Vec::with_capacity(k);

    // convert kmeans-id back to pbfextractor-id
    for cluster in kmeans.clusters() {
        for id in cluster.1 {
            order.push(node_ids[id]);
        }
        // save end index as end of cluster
        offsets.push(order.len());
    }

    Ok((offsets, order))
}

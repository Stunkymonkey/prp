use super::*;
use max_heap::MaxHeapItem;

use rand::{rngs::StdRng, Rng, SeedableRng};
use std::collections::{BTreeSet, BinaryHeap};
use std::time::Instant;

// TODO: test if edge removal of inner edges is better
// TODO: test not using sets is faster?

pub fn merge(
    partition_amounts: &mut Vec<usize>,
    partition_sizes: &[usize],
    nodes: &mut Vec<Node>,
    edges: &[Edge],
) -> Result<(), String> {
    let heuristic_time = Instant::now();

    // create offsets
    let mut up_offset = Vec::<EdgeId>::new();
    up_offset.resize(nodes.len() + 1, 0);
    let sources: Vec<EdgeId> = edges.par_iter().map(|x| x.from).rev().collect();
    offset::fill_offset(sources, &mut up_offset);

    // vector of optional Heaps to store what nodes are in each partition
    let mut sets: Vec<BTreeSet<NodeId>> = Vec::with_capacity(nodes.len());
    // minheap for heuristics with nodeId and heuristic value
    let mut heap: BinaryHeap<MaxHeapItem> = BinaryHeap::new();

    // for consistent deterministic randomness
    let mut rng: Box<StdRng> = Box::new(SeedableRng::seed_from_u64(42));

    // keep track whats main of partition
    let mut current_partition = Vec::with_capacity(nodes.len());
    // keep track if heuristics are up-to-date
    let mut amount_merges = Vec::with_capacity(nodes.len());

    // counting the amount of partitions
    let mut partition_amount = nodes.len();

    // keep track of how many_layers are done
    let mut amount_layer: usize = 0;

    // save current_partition and amount of partitions
    let mut results: Vec<(Vec<BTreeSet<NodeId>>, Vec<PartitionId>, usize)> = Vec::new();

    for node_id in 0..nodes.len() {
        let mut new_partiton = BTreeSet::new();
        new_partiton.insert(node_id);
        sets.push(new_partiton);

        // set each node as its own partition
        current_partition.push(node_id);
        amount_merges.push(0);
    }

    // calculate heuristics of all neighbor partitions and insert them in the heap
    // to prevent multiple calculations of neighbors the set is used
    let mut already_inserted: BTreeSet<(NodeId, NodeId)> = BTreeSet::new();
    for node_id in 0..nodes.len() {
        // since graph is bidirectional up-graph is sufficient
        let neighbors = graph_helper::get_up_neighbors(node_id, &edges, &up_offset);
        for neighbor in neighbors {
            if !already_inserted.contains(&(node_id, neighbor)) {
                heap.push(MaxHeapItem::new(
                    (node_id, neighbor),
                    (0, 0),
                    get_priority(
                        node_id,
                        neighbor,
                        &mut rng,
                        &sets,
                        &current_partition,
                        &edges,
                        &up_offset,
                    ),
                ));

                already_inserted.insert((neighbor, node_id));
            }
        }
    }
    drop(already_inserted);

    println!("initial heuristic time in: {:?}", heuristic_time.elapsed());

    let merge_time = Instant::now();

    let mut heuristic_sum: f64 = 0.0;

    while let Some(MaxHeapItem {
        partition_ids,
        amount_merged,
        heuristic,
    }) = heap.pop()
    {
        let (set_a, set_b) = partition_ids;
        let (a_merges, b_merges) = amount_merged;

        // skip if one partiton has been merged with another set
        if amount_merges[set_a] != a_merges || amount_merges[set_b] != b_merges {
            continue;
        }

        // update ids of each node
        for node in &sets[set_b] {
            current_partition[*node] = set_a;
        }
        amount_merges[set_a] += 1;
        amount_merges[set_b] += 1;
        // combine both sets
        sets[set_a] = sets[set_a].union(&sets[set_b]).cloned().collect();
        sets[set_b] = BTreeSet::new();

        heuristic_sum += heuristic;

        // collect all neighbors
        let mut neighbors: Vec<NodeId> = sets[set_a]
            .par_iter()
            .map(|node| graph_helper::get_up_neighbors(*node, &edges, &up_offset))
            .flatten()
            .collect();
        neighbors.par_sort_unstable();
        neighbors.dedup();

        // collect all neighbor sets
        let mut neighbor_sets: Vec<PartitionId> = neighbors
            .par_iter()
            .map(|neighbor| current_partition[*neighbor])
            .collect();
        neighbor_sets.par_sort_unstable();
        neighbor_sets.dedup();

        // update heuristics for neighboring sets
        for neighbor_set in neighbor_sets {
            // do not insert heuristics to merged set again
            if neighbor_set == set_a || neighbor_set == set_b {
                continue;
            }
            heap.push(MaxHeapItem::new(
                (set_a, neighbor_set),
                (amount_merges[set_a], amount_merges[neighbor_set]),
                get_priority(
                    set_a,
                    neighbor_set,
                    &mut rng,
                    &sets,
                    &current_partition,
                    &edges,
                    &up_offset,
                ),
            ));
        }

        partition_amount -= 1;
        let partition_size = sets[set_a].len();

        // if partition_amount % 1000 == 0 {
        //     println!("partition_amount {:?}", partition_amount);
        // }

        // check if saving of one layer is needed
        if (!partition_sizes.is_empty() && partition_sizes[amount_layer] <= partition_size)
            || (!partition_amounts.is_empty()
                && partition_amounts
                    .iter()
                    .take(partition_amounts.len() - amount_layer)
                    .product::<usize>()
                    >= partition_amount)
        {
            amount_layer += 1;
            results.push((sets.clone(), current_partition.clone(), partition_amount));
        }
        // now the layers are all finished
        if amount_layer >= std::cmp::max(partition_sizes.len(), partition_amounts.len()) {
            // println!(
            //     "nodes: {:?}\t upper-partition_amount: {:?}\t max-partition_size: {:?}",
            //     nodes.len(),
            //     partition_amount,
            //     partition_size
            // );
            break;
        }
    }
    drop(sets);
    drop(current_partition);

    // write new sizes back (if option `-p` is used it should be the same as before)
    let mut tmp: Vec<usize> = results
        .iter()
        .rev()
        .map(|(_, _, partition_amount)| *partition_amount)
        .collect();

    // divide, to get divider-numbers
    let old_values = tmp.clone();
    for (index, amount) in tmp.iter_mut().skip(1).enumerate() {
        // round up division
        *amount = *amount / old_values[index] + (*amount % old_values[index] != 0) as usize;
    }
    if !partition_amounts.is_empty() {
        assert_eq!(tmp.len(), partition_amounts.len());
    }
    *partition_amounts = tmp;

    // TODO test if correct
    for node in nodes.into_iter() {
        node.partition = 0;
    }
    for (layer, (sets, current_partition, _)) in results.iter().rev().enumerate() {
        let multiplier = partition_amounts
            .iter()
            .skip(1)
            .take(partition_amounts.len() - layer)
            .product::<usize>();
        // println!("multiplier {:?}", multiplier);
        for (s, set) in sets.iter().enumerate() {
            // skip empty sets
            if set.is_empty() {
                continue;
            }
            // get the id of the current set
            let previous_id = nodes[set.iter().next().unwrap().clone()].partition;

            // set all nodes
            for node in set {
                nodes[*node].partition = previous_id + s * multiplier;
            }
        }
    }

    println!(
        "merge_time time in: {:?} with value {:?}",
        merge_time.elapsed(),
        heuristic_sum as usize
    );

    Ok(())
}

fn get_priority(
    node_id_a: NodeId,
    node_id_b: NodeId,
    rng: &mut Box<StdRng>,
    sets: &[BTreeSet<NodeId>],
    _current_partition: &[PartitionId],
    edges: &[Edge],
    up_offset: &[EdgeId],
) -> f64 {
    // TODO maybe collect nodes by iterating one time over all nodes and collect them
    let nodes_a = &sets[node_id_a];
    let nodes_b = &sets[node_id_b];

    // let mut nodes_a: Vec<NodeId> = current_partition
    //     .par_iter()
    //     .enumerate()
    //     .filter(|(_id, partition)| **partition == node_id_a)
    //     .map(|(id, _partition)| id)
    //     .collect();
    // nodes_a.par_sort_unstable();

    // count amount of edges between both sets (bidirectional -> onsoly one side-comparison)
    let mut connecting_a_b = 0;
    for node in nodes_a {
        let neighbors = graph_helper::get_up_neighbors(*node, &edges, &up_offset);
        for neighbor in neighbors {
            if nodes_b.contains(&neighbor) {
                connecting_a_b += 1;
            }
        }
    }

    // amount of nodes that have a connection to outside
    let mut border_nodes_a = 0;
    for node in nodes_a {
        let node_neighbors = graph_helper::get_up_neighbors(*node, &edges, &up_offset);
        for neighbor in node_neighbors {
            if !nodes_a.contains(&neighbor) {
                border_nodes_a += 1;
                break;
            }
        }
    }

    // amount of nodes that have a connection to outside
    let mut border_nodes_b = 0;
    for node in nodes_b {
        let node_neighbors = graph_helper::get_up_neighbors(*node, &edges, &up_offset);
        for neighbor in node_neighbors {
            if !nodes_b.contains(&neighbor) {
                border_nodes_b += 1;
                break;
            }
        }
    }

    // amount of nodes that have a connection outside AFTER union of sets
    let mut new_border_nodes = 0;
    for node in nodes_a {
        let node_neighbors = graph_helper::get_up_neighbors(*node, &edges, &up_offset);
        for neighbor in node_neighbors {
            if !(nodes_a.contains(&neighbor) || nodes_b.contains(&neighbor)) {
                new_border_nodes += 1;
                break;
            }
        }
    }
    for node in nodes_b {
        let node_neighbors = graph_helper::get_up_neighbors(*node, &edges, &up_offset);
        for neighbor in node_neighbors {
            if !(nodes_a.contains(&neighbor) || nodes_b.contains(&neighbor)) {
                new_border_nodes += 1;
                break;
            }
        }
    }

    (connecting_a_b * (1 + border_nodes_a + border_nodes_b + new_border_nodes)) as f64
        / (nodes_a.len() * nodes_b.len()) as f64
        * rng.gen_range(1.0..1.01)
}

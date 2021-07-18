use super::*;
use max_heap::MaxHeapItem;

use rand::{rngs::StdRng, Rng, SeedableRng};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap};
use std::time::Instant;

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
    let sources: Vec<EdgeId> = edges.iter().map(|x| x.from).rev().collect();
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
    let mut amount_merges = vec![0; nodes.len()];
    let mut amount_merges_per_level = vec![0; nodes.len()];

    // counting the amount of partitions
    let mut partition_amount = nodes.len();

    // keep track of how many levels are done
    let mut amount_level: usize = 0;

    // save current_partition and amount of partitions
    let mut results: Vec<(Vec<BTreeSet<NodeId>>, usize, usize)> = Vec::with_capacity(
        std::cmp::max(partition_amounts.len(), partition_sizes.len()),
    );

    for node_id in 0..nodes.len() {
        let mut new_partiton = BTreeSet::new();
        new_partiton.insert(node_id);
        sets.push(new_partiton);

        // set each node as its own partition
        current_partition.push(node_id);
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

    println!("initial heuristic in: {:?}", heuristic_time.elapsed());

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
        let new_value = amount_merges_per_level[set_a] + amount_merges_per_level[set_b] + 1;
        amount_merges_per_level[set_a] = new_value;
        amount_merges_per_level[set_b] = new_value;
        // combine both sets and switch so the small get merged into the bigger
        let mut merge_set = std::mem::take(&mut sets[set_b]);
        if merge_set.len() > sets[set_a].len() {
            std::mem::swap(&mut sets[set_a], &mut merge_set);
        }
        sets[set_a].extend(merge_set);

        heuristic_sum += heuristic;

        // collect all neighbors
        let mut neighbors: Vec<NodeId> = sets[set_a]
            .iter()
            .map(|node| graph_helper::get_up_neighbors(*node, &edges, &up_offset))
            .flatten()
            .collect();
        neighbors.par_sort_unstable();
        neighbors.dedup();

        // collect all neighbor sets
        let mut neighbor_sets: Vec<PartitionId> = neighbors
            .iter()
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

        // check if saving of one level is needed
        if (!partition_sizes.is_empty() && partition_sizes[amount_level] <= partition_size)
            || (!partition_amounts.is_empty()
                && partition_amounts[amount_level] >= partition_amount)
        {
            println!(
                "one level finished with {:?} partitions and a maximum partition-size of {:?}",
                partition_amount, partition_size
            );
            amount_level += 1;
            results.push((
                sets.clone(),
                partition_amount,
                amount_merges_per_level.iter().max().unwrap_or(&0) + 1,
            ));
            amount_merges_per_level = vec![0; nodes.len()];
        }
        // now the levels are all finished
        if amount_level >= std::cmp::max(partition_sizes.len(), partition_amounts.len()) {
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

    // get last partition amount number
    *partition_amounts = vec![results
        .iter()
        .map(|(_, partition_amount, _)| *partition_amount)
        .last()
        .unwrap_or(1)];

    // append amount of merges in each level and ignore first merges
    results
        .iter()
        .skip(1)
        .rev()
        .for_each(|(_, _, merge_amount)| partition_amounts.push(*merge_amount));

    // for checking if all ids are not exceeding the maximum (product)
    let mut maximum_id: usize = 1;
    for partition_amount in partition_amounts.iter_mut() {
        maximum_id = match maximum_id.checked_mul(*partition_amount) {
            Some(value) => value,
            None => {
                panic!("overflow occured: maximum id exceeds 64bit");
            }
        }
    }

    for node in nodes.iter_mut() {
        node.partition = 0;
    }
    // assign ids
    for (i, (sets, _, _)) in results.iter().rev().enumerate() {
        // calculate new offset
        let new_offset = partition_amounts.iter().skip(i + 1).product::<usize>();

        // to keep track how often each interval has been assigned
        let mut set_amount_counter: BTreeMap<usize, usize> = BTreeMap::new();
        for set in sets {
            // skip empty sets
            if set.is_empty() {
                continue;
            }
            // get the id of the set previously assigned
            let previous_id = nodes[*set.iter().next().unwrap()].partition;

            set_amount_counter.entry(previous_id).or_insert(0);

            // calculate the new id and check for overflow
            let new_set_id = match previous_id
                .checked_add(new_offset * set_amount_counter.get(&previous_id).unwrap())
            {
                Some(value) => value,
                None => {
                    panic!("overflow occured, while assinging partition-ids");
                }
            };

            // check if ids are exceeting the maximum
            assert!(
                *set_amount_counter.get(&previous_id).unwrap() < partition_amounts[i],
                "more sub partitions as expected"
            );
            assert!(
                new_set_id <= maximum_id,
                "export-partition-id is too big: max {:?} value {:?}",
                maximum_id,
                new_set_id
            );

            // set all nodes
            for node in set {
                nodes[*node].partition = new_set_id;
            }
            // increase counter
            if let Some(counter) = set_amount_counter.get_mut(&previous_id) {
                *counter += 1;
            }
        }
    }

    partition_amounts.reverse();

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
    let nodes_a = &sets[node_id_a];
    let nodes_b = &sets[node_id_b];

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

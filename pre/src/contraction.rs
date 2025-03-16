use super::*;
use ndijkstra::NDijkstra;
use valid_flag::ValidFlag;

use crossbeam_channel::bounded;
use mch::same_array;
use std::cmp::Reverse;
use std::collections::BTreeSet;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

fn sort_nodes_ranked(
    edges: &mut Vec<Edge>,
    up_offset: &[EdgeId],
    down_offset: &[EdgeId],
    nodes: &mut [Node],
) {
    // sort nodes based on level & rank & edge-degree so hopefully only forward walking is done in dijkstra
    nodes
        .par_iter_mut()
        .enumerate()
        .for_each(|(i, node)| node.old_id = Some(i));
    nodes.par_sort_unstable_by(|a, b| {
        a.level.cmp(&b.level).then(a.rank.cmp(&b.rank)).then(
            graph_helper::node_degree(a.old_id.unwrap(), up_offset, down_offset).cmp(
                &graph_helper::node_degree(b.old_id.unwrap(), up_offset, down_offset),
            ),
        )
    });
    // create new index
    let mut new_node_index = vec![INVALID_NODE; nodes.len()];
    nodes
        .iter()
        .enumerate()
        .for_each(|(i, node)| new_node_index[node.old_id.unwrap()] = i);

    // iterate over edges and fix all from/to ids
    edges.par_iter_mut().for_each(|edge| {
        edge.to = new_node_index[edge.to];
        edge.from = new_node_index[edge.from];
    });
}

fn sort_edges_ranked(
    edges: &mut Vec<Edge>,
    down_offset: &[EdgeId],
    down_index: &mut Vec<EdgeId>,
    nodes: &[Node],
) {
    //sort by source then rank
    edges.par_sort_unstable_by(|a, b| {
        a.from
            .cmp(&b.from)
            .then(nodes[a.to].rank.cmp(&nodes[b.to].rank).reverse())
    });

    *down_index = vec![INVALID_EDGE; edges.len()];
    // fill offsets, where not already filled
    for (i, edge) in edges.iter().enumerate() {
        let start_index = down_offset[edge.to];
        let end_index = down_offset[edge.to + 1];
        for j in down_index.iter_mut().take(end_index).skip(start_index) {
            if *j == INVALID_EDGE {
                *j = i;
                break;
            }
        }
    }

    // sort down_index subvectors
    for node in 0..nodes.len() {
        let subvector = &mut down_index[down_offset[node]..down_offset[node + 1]];
        subvector.sort_unstable_by_key(|edge_id| Reverse(nodes[edges[*edge_id].from].rank));
    }
}

fn revert_indices(edges: &mut Vec<Edge>) {
    let maximum_id = edges
        .par_iter()
        .map(|edge| edge.id)
        .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .unwrap()
        .unwrap();
    let mut indices = vec![INVALID_NODE; maximum_id + 1];

    for (i, edge) in edges.iter().enumerate() {
        indices[edge.id.unwrap()] = i;
    }

    edges.par_iter_mut().for_each(|edge| {
        if edge.contracted_edges.is_some() {
            edge.contracted_edges = Some((
                indices[edge.contracted_edges.unwrap().0],
                indices[edge.contracted_edges.unwrap().1],
            ));
        }
    });

    // check if indices are not invalid
    for edge in edges {
        if let Some((prev, next)) = edge.contracted_edges {
            assert!(prev != INVALID_NODE, "at least one index is invalid");
            assert!(next != INVALID_NODE, "at least one index is invalid");
        }
    }
}

// contract one level until end
#[allow(clippy::too_many_arguments)]
fn level_contraction(
    level: Level,
    remaining_nodes: &mut BTreeSet<NodeId>,
    independent_set_flags: &mut ValidFlag,
    heuristics: &mut [usize],
    deleted_neighbors: &mut [usize],
    shortcut_id: &AtomicUsize,
    resulting_edges: &mut Vec<Edge>,
    rank: &mut Rank,
    nodes: &mut [Node],
    edges: &mut Vec<Edge>,
    up_offset: &mut Vec<EdgeId>,
    down_offset: &mut Vec<EdgeId>,
    down_index: &mut Vec<EdgeId>,
    mut contracted_nodes_amount: usize,
    contraction_stop: f64,
) {
    let dim = edges[0].cost.len();
    let amount_nodes = nodes.len();

    let thread_count = num_cpus::get();

    while !(remaining_nodes.is_empty()
        || (contracted_nodes_amount as f64 / nodes.len() as f64) > contraction_stop)
    {
        // I ← independent node set
        let minimas = ordering::get_independent_set(
            remaining_nodes,
            heuristics,
            independent_set_flags,
            edges,
            up_offset,
            down_offset,
            down_index,
        );

        // REMINDER: the break is allowed and does not mess up correctness only on the top layer
        // if minimas.len() == 1 {
        //     break;
        // }

        // E ← necessary shortcuts

        // channels for sending and vector for collecting results
        let (node_send, node_recieve) = bounded(0);
        let parallel_shortcuts: Mutex<Vec<Edge>> =
            Mutex::new(Vec::with_capacity(dim * minimas.len()));

        rayon::scope(|s| {
            for _thread in 0..thread_count {
                s.spawn(|_| {
                    let mut dijkstra = NDijkstra::new(amount_nodes, dim);
                    // REMINDER: check process-fork slow down
                    let mut mch = match mch::Contractor::new(
                        // dijkstra
                        |start, end, alpha| -> Vec<Cost> {
                            match dijkstra.find_path(start, end, alpha.to_vec(), up_offset, edges) {
                                Some(costs) => costs.1,
                                None => vec![COST_MAX; edges[0].cost.len()],
                            }
                        },
                        // to-edges
                        |node_id| -> Vec<mch::Edge<EdgeId, NodeId>> {
                            let down_edge_ids =
                                graph_helper::get_down_edge_ids(node_id, down_offset, down_index);
                            let mut mch_edges = Vec::new();
                            for down_edge_id in down_edge_ids {
                                let edge = &edges[down_edge_id];
                                mch_edges.push(mch::Edge::new(
                                    edge.id.unwrap(),
                                    edge.from,
                                    edge.to,
                                    edge.cost.clone(),
                                ))
                            }
                            mch_edges
                        },
                        // from-edges
                        |node_id| -> Vec<mch::Edge<EdgeId, NodeId>> {
                            let up_edge_ids = graph_helper::get_up_edge_ids(node_id, up_offset);
                            let mut mch_edges = Vec::new();
                            for up_edge_id in up_edge_ids {
                                let edge = &edges[up_edge_id];
                                mch_edges.push(mch::Edge::new(
                                    edge.id.unwrap(),
                                    edge.from,
                                    edge.to,
                                    edge.cost.clone(),
                                ))
                            }
                            mch_edges
                        },
                        //dims
                        dim,
                    ) {
                        Ok(mch) => mch,
                        Err(error) => {
                            panic!("error with mch: '{:?}'", error);
                        }
                    };

                    let mut thread_shortcuts =
                        Vec::with_capacity((dim * minimas.len()) / thread_count);
                    while let Ok((to, from)) = node_recieve.recv() {
                        let mch_shortcut = match mch.shortcuts(&to, &from) {
                            Ok(ok) => ok,
                            Err(err) => panic!("contraction error: '{:?}'", err),
                        };
                        if let Some(shortcut) = mch_shortcut {
                            thread_shortcuts.push(Edge::shortcut(
                                shortcut.from,
                                shortcut.to,
                                shortcut.cost,
                                shortcut_id.fetch_add(1, Ordering::SeqCst),
                                shortcut.replaced_edges,
                            ));
                        }
                    }

                    let mut tmp = parallel_shortcuts.lock().unwrap();
                    tmp.extend(thread_shortcuts);
                });
            }
            for node_id in &minimas {
                for to_edge_id in graph_helper::get_down_edge_ids(*node_id, down_offset, down_index)
                {
                    let edge = &edges[to_edge_id];
                    let to_edge =
                        mch::Edge::new(edge.id.unwrap(), edge.from, edge.to, edge.cost.clone());
                    for from_edge_id in graph_helper::get_up_edge_ids(*node_id, up_offset) {
                        let edge = &edges[from_edge_id];
                        let from_edge =
                            mch::Edge::new(edge.id.unwrap(), edge.from, edge.to, edge.cost.clone());
                        node_send.send((to_edge.clone(), from_edge)).unwrap();
                    }
                }
            }
            drop(node_send);
        });

        let mut shortcuts = parallel_shortcuts.into_inner().unwrap();

        // collecting all edges to be removed
        let mut connected_edges: Vec<EdgeId> = minimas
            .par_iter()
            .map(|node| graph_helper::get_all_edge_ids(*node, up_offset, down_offset, down_index))
            .flatten()
            .collect();

        // dedup shortcuts; preventing shortcuts in diamond-shapes
        shortcuts.par_sort_unstable_by(|a, b| {
            a.from
                .cmp(&b.from)
                .then(a.to.cmp(&b.to))
                .then(a.cost.partial_cmp(&(b.cost)).unwrap())
        });
        // only dedup exakt duplicates
        shortcuts.dedup_by(|a, b| a.from == b.from && a.to == b.to && same_array(&a.cost, &b.cost));

        // update heuristic of neighbors of I with simulated contractions
        let mut neighbors: Vec<NodeId> = minimas
            .par_iter()
            .map(|node| {
                graph_helper::get_all_neighbours(*node, edges, up_offset, down_offset, down_index)
            })
            .flatten()
            .collect();
        for neighbor in &neighbors {
            deleted_neighbors[*neighbor] += 1;
        }
        neighbors.par_sort_unstable();
        neighbors.dedup();
        ordering::update_neighbor_heuristics(
            neighbors,
            level,
            heuristics,
            nodes,
            deleted_neighbors,
            up_offset,
            down_offset,
        );

        // sort in reverse order for removing from bottom up
        connected_edges.par_sort_unstable_by_key(|&edge| Reverse(edge));
        // insert E into remaining graph
        for edge_id in connected_edges.iter() {
            resulting_edges.push(edges.swap_remove(*edge_id));
        }

        // add new shortcuts to edges
        let amount_shortcuts = shortcuts.len();
        edges.par_extend(shortcuts);

        // recalc edge-indices
        *down_index = offset::generate_offsets(edges, up_offset, down_offset, nodes.len());

        // move I to their rank
        for node in &minimas {
            nodes[*node].rank = *rank;
            remaining_nodes.remove(node);
        }
        contracted_nodes_amount += minimas.len();

        println!(
            "rank {:?}  \tremaining_nodes {:?} \tindependent_set {:?} \tedges {:?} \tshortcuts {:?}     \tremoving_edges {:?} \tresulting_edges {:?}",
            rank,
            remaining_nodes.len(),
            minimas.len(),
            edges.len(),
            amount_shortcuts,
            connected_edges.len(),
            resulting_edges.len()
        );
        *rank += 1;
    }
}

pub fn prp_contraction(
    nodes: &mut [Node],
    edges: &mut Vec<Edge>,
    up_offset: &mut Vec<EdgeId>,
    down_offset: &mut Vec<EdgeId>,
    down_index: &mut Vec<EdgeId>,
    mlp_levels: &[usize],
    contraction_stop: f64,
) {
    let mut independent_set_flags = ValidFlag::new(nodes.len());

    let mut deleted_neighbors = vec![0; nodes.len()];

    let mut rank: Rank = 0;

    // edge-ids
    let shortcut_id = AtomicUsize::new(edges.len());
    // make edges have indices
    edges
        .par_iter_mut()
        .enumerate()
        .for_each(|(i, x)| x.id = Some(i));

    let contracted_nodes_amount = 0;

    let mut resulting_edges = Vec::<Edge>::with_capacity(edges.len() * mlp_levels.len());

    for level in 0..mlp_levels.len() {
        let mut remaining_nodes = BTreeSet::new();
        for (node_id, node) in nodes.iter().enumerate() {
            if level == node.level {
                remaining_nodes.insert(node_id);
            }
        }

        // assign core-edge
        for edge in edges.iter_mut() {
            edge.level = Some(level);
        }

        let mut heuristics = ordering::calculate_heuristics(
            level,
            nodes,
            &deleted_neighbors,
            up_offset,
            down_offset,
        );
        level_contraction(
            level,
            &mut remaining_nodes,
            &mut independent_set_flags,
            &mut heuristics,
            &mut deleted_neighbors,
            &shortcut_id,
            &mut resulting_edges,
            &mut rank,
            nodes,
            edges,
            up_offset,
            down_offset,
            down_index,
            contracted_nodes_amount,
            contraction_stop,
        );
    }

    // testing uniqueness of ids
    let unique_set: BTreeSet<usize> = edges.iter().cloned().map(|e| e.id.unwrap()).collect();
    assert_eq!(unique_set.len(), edges.len());

    // assign each top edge to top level
    for edge in edges.iter_mut() {
        edge.level = Some(mlp_levels.len());
    }

    // merging both graphs back together to have a single one
    edges.par_extend(resulting_edges);

    // check that no edge has invalid height
    for edge in edges.iter_mut() {
        if let Some(level) = edge.level {
            assert!(INVALID_LEVEL != level);
        }
    }

    // check that levels are correct
    if contraction_stop < 1.0 {
        let unique_height_set: BTreeSet<usize> =
            edges.iter().cloned().filter_map(|e| e.level).collect();
        assert!(unique_height_set.len() - 1 == mlp_levels.len());
    }

    sort_nodes_ranked(edges, up_offset, down_offset, nodes);

    // and calculate the offsets
    *down_index = offset::generate_offsets(edges, up_offset, down_offset, nodes.len());

    // sort edges from top to down ranks for pch-dijkstra
    sort_edges_ranked(edges, down_offset, down_index, nodes);

    // revert the edge-ids back to usual ids
    revert_indices(edges);
}

#[test]
fn revert_indices_test() {
    //      7 -> 8 -> 9
    //      |         |
    // 0 -> 5 -> 6 -  |
    // |         |  \ |
    // 1 -> 2 -> 3 -> 4

    let amount_nodes = 10;
    let mut edges = vec![
        Edge::test(6, 4, vec![20.0], 8),
        Edge::test(6, 3, vec![20.0], 7),
        Edge::test(5, 6, vec![1.0], 9),
        Edge::test(5, 7, vec![5.0], 6),
        Edge::shortcut(5, 3, vec![21.0], 12, (9, 7)),
        Edge::test(0, 5, vec![5.0], 0),
        Edge::test(3, 4, vec![20.0], 1),
        Edge::test(2, 3, vec![1.0], 2),
        Edge::test(8, 9, vec![1.0], 11),
        Edge::test(7, 8, vec![1.0], 5),
        Edge::test(1, 2, vec![1.0], 3),
        Edge::test(0, 1, vec![1.0], 4),
        Edge::shortcut(0, 7, vec![10.0], 13, (0, 6)),
        Edge::shortcut(0, 2, vec![2.0], 16, (4, 3)),
        Edge::test(9, 4, vec![1.0], 10),
        Edge::shortcut(7, 9, vec![2.0], 15, (5, 11)),
        Edge::shortcut(7, 4, vec![3.0], 17, (15, 10)),
        Edge::shortcut(2, 4, vec![21.0], 14, (2, 1)),
    ];

    let mut up_offset = Vec::<EdgeId>::new();
    let mut down_offset = Vec::<EdgeId>::new();
    let _down_index =
        offset::generate_offsets(&mut edges, &mut up_offset, &mut down_offset, amount_nodes);

    let expected_edges = vec![
        Edge::test(0, 1, vec![1.0], 4),
        Edge::shortcut(0, 2, vec![2.0], 16, (0, 4)),
        Edge::test(0, 5, vec![5.0], 0),
        Edge::shortcut(0, 7, vec![10.0], 13, (2, 10)),
        Edge::test(1, 2, vec![1.0], 3),
        Edge::test(2, 3, vec![1.0], 2),
        Edge::shortcut(2, 4, vec![21.0], 14, (5, 7)),
        Edge::test(3, 4, vec![20.0], 1),
        Edge::shortcut(5, 3, vec![21.0], 12, (9, 11)),
        Edge::test(5, 6, vec![1.0], 9),
        Edge::test(5, 7, vec![5.0], 6),
        Edge::test(6, 3, vec![20.0], 7),
        Edge::test(6, 4, vec![20.0], 8),
        Edge::shortcut(7, 4, vec![3.0], 17, (15, 17)),
        Edge::test(7, 8, vec![1.0], 5),
        Edge::shortcut(7, 9, vec![2.0], 15, (14, 16)),
        Edge::test(8, 9, vec![1.0], 11),
        Edge::test(9, 4, vec![1.0], 10),
    ];

    revert_indices(&mut edges);

    assert_eq!(edges, expected_edges);
}

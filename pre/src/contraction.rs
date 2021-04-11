use super::*;
use ndijkstra::NDijkstra;
use valid_flag::ValidFlag;

use std::collections::BTreeSet;

// contract one layer until end
#[allow(clippy::too_many_arguments)]
fn layer_contraction(
    remaining_nodes: &mut BTreeSet<NodeId>,
    mut independent_set_flags: &mut ValidFlag,
    heuristics: &mut Vec<usize>,
    nodes: &mut Vec<Node>,
    mut edges: &mut Vec<Edge>,
    mut up_offset: &mut Vec<EdgeId>,
    mut down_offset: &mut Vec<EdgeId>,
    mut down_index: &mut Vec<EdgeId>,
) {
    while !remaining_nodes.is_empty() {
        let get_independent_set_time = Instant::now();
        // I ← independent node set
        let mut minimas = ordering::get_independent_set(
            &remaining_nodes,
            &heuristics,
            &mut independent_set_flags,
            &edges,
            &up_offset,
            &down_offset,
            &down_index,
        );
        if remaining_nodes.len() > 100_000 {
            println!(
                "get_independent_set time in: {:?}",
                get_independent_set_time.elapsed()
            );
        }
        let mut mch = match mch::Contractor::new(
            // dijkstra
            |start, end, alpha| -> Vec<Cost> {
                let mut d = NDijkstra::new(nodes.len(), edges[0].cost.len());
                match d.find_path(start, end, alpha.to_vec(), &up_offset, &edges, false) {
                    Some(costs) => vec![costs.1],
                    None => vec![COST_MAX],
                }
            },
            // to-edges
            |node_id| -> Vec<mch::Edge<EdgeId, NodeId>> {
                let up_edge_ids = graph_helper::get_up_edge_ids(node_id, &up_offset);
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
            // from-edges
            |node_id| -> Vec<mch::Edge<EdgeId, NodeId>> {
                let down_edge_ids =
                    graph_helper::get_down_edge_ids(node_id, &down_offset, &down_index);
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
            //dims
            edges[0].cost.len(),
        ) {
            Ok(mch) => mch,
            Err(error) => {
                panic!("error with mch (glpk): '{:?}'", error);
            }
        };
        println!(
            "mch::contract{:?}",
            mch.contract(*remaining_nodes.iter().next().unwrap())
        );

        /*
        contract idependent set in parallel
        save new edges to new list
        make new graph (unsure about vectors)
        */
        break;
    }
}

//TODO
pub fn prp_contraction(
    nodes: &mut Vec<Node>,
    mut edges: &mut Vec<Edge>,
    mut up_offset: &mut Vec<EdgeId>,
    mut down_offset: &mut Vec<EdgeId>,
    mut down_index: &mut Vec<EdgeId>,
    mlp_layers: &[usize],
) {
    let mut independent_set_flags = ValidFlag::new(nodes.len());
    let mut heuristics = ordering::calculate_heuristics(nodes.len(), &up_offset, &down_offset);

    // make edges have indices
    edges
        .par_iter_mut()
        .enumerate()
        .for_each(|(i, x)| x.id = Some(i));

    for layer_height in 0..=mlp_layers.len() {
        let mut remaining_nodes = BTreeSet::new();
        for (node_id, node) in nodes.iter().enumerate() {
            if layer_height == node.layer_height {
                remaining_nodes.insert(node_id);
            }
        }
        layer_contraction(
            &mut remaining_nodes,
            &mut independent_set_flags,
            &mut heuristics,
            nodes,
            &mut edges,
            &mut up_offset,
            &mut down_offset,
            &mut down_index,
        )
    }

    //sort edges within offsets first based on layer_height then on rank
    // edges.par_sort_by(|a, b| {
    //     a.source
    //         .cmp(&b.source)
    //         .then(nodes[a.target].layer_height.cmp(&nodes[b.target].layer_height).reverse())
    //         .then(nodes[a.target].rank.cmp(&nodes[b.target].rank).reverse())
    // });
}

use super::*;
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
    mlp_layers: &[usize],
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
        /*
        contract idependent set in parallel without looking what partition it really is
        save new edges to new list
        make new graph (unsure about vectors)
        */
        // mch::Contractor::new(
        //     dijkstra: D,
        //     to_edges: ToEdges,
        //     from_edges: FromEdges,
        //     dim: usize,
        // );
        break;
    }
}

//TODO
pub fn pch_contraction(
    nodes: &mut Vec<Node>,
    mut edges: &mut Vec<Edge>,
    mut up_offset: &mut Vec<EdgeId>,
    mut down_offset: &mut Vec<EdgeId>,
    mut down_index: &mut Vec<EdgeId>,
    mlp_layers: &[usize],
) {
    let mut independent_set_flags = ValidFlag::new(nodes.len());
    let mut heuristics = ordering::calculate_heuristics(nodes.len(), &up_offset, &down_offset);

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
            &mlp_layers,
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

use super::*;

/// fill offset array
pub fn fill_offset(edges: Vec<NodeId>, offset: &mut Vec<usize>) {
    for edge in edges {
        offset[edge + 1] += 1;
    }
    for i in 1..offset.len() {
        offset[i] += offset[i - 1];
    }
}

/// make sure edges are already sorted!!
#[allow(dead_code)]
pub fn generate_offsets_unstable(
    edges: &mut Vec<Edge>,
    mut up_offset: &mut Vec<EdgeId>,
    mut down_offset: &mut Vec<EdgeId>,
    amount_nodes: usize,
) -> Vec<EdgeId> {
    up_offset.clear();
    up_offset.resize(amount_nodes + 1, 0);
    down_offset.clear();
    down_offset.resize(amount_nodes + 1, 0);

    // generate up edges
    let sources: Vec<EdgeId> = edges.par_iter().map(|x| x.from).rev().collect();
    fill_offset(sources, &mut up_offset);

    // generate down edges, but without sorting edges
    // first collect offsets
    let targets: Vec<EdgeId> = edges.par_iter().map(|x| x.to).rev().collect();
    fill_offset(targets, &mut down_offset);
    let mut down_index = vec![INVALID_EDGE; edges.len()];
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
    down_index
}

#[allow(dead_code)]
pub fn generate_offsets(
    edges: &mut Vec<Edge>,
    up_offset: &mut Vec<EdgeId>,
    down_offset: &mut Vec<EdgeId>,
    amount_nodes: usize,
) -> Vec<EdgeId> {
    edges.par_sort_unstable();
    generate_offsets_unstable(edges, up_offset, down_offset, amount_nodes)
}

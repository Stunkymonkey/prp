use super::*;
use std::cmp::Reverse;

pub fn sort_edges(method: QueryType, data: &mut BinFile) {
    let dim = data.edge_costs.len() / data.edges.len();
    match method {
        QueryType::Normal | QueryType::Bi => {
            // add id to keep track of permuation
            set_indices(&mut data.edges);

            // sort only level zero
            for node in 0..data.nodes.len() {
                let subvector = &mut data.edges[data.up_offset[node]..data.up_offset[node + 1]];
                subvector.sort_unstable_by_key(|edge| edge.contracted_edges);
            }

            // get permutation
            let new_edge_index = get_indices(&data.edges);

            // clear index
            clear_indices(&mut data.edges);

            // sort costs according to permutation
            fix_cost(&mut data.edge_costs, &new_edge_index, dim);

            // resolve pch-indices
            fix_contracted_edges(&mut data.edges, &new_edge_index);

            // down_index update
            let edges = &data.edges;
            for node in 0..data.nodes.len() {
                let subvector =
                    &mut data.down_index[data.down_offset[node]..data.down_offset[node + 1]];
                subvector
                    .iter_mut()
                    .for_each(|edge_id| *edge_id = new_edge_index[*edge_id]);
                subvector.sort_unstable_by_key(|edge_id| edges[*edge_id].contracted_edges);
            }
        }
        QueryType::Pch => {
            // add id to keep track of permuation
            set_indices(&mut data.edges);

            // sort by rank
            let nodes = &data.nodes;
            for node in 0..data.nodes.len() {
                let subvector = &mut data.edges[data.up_offset[node]..data.up_offset[node + 1]];
                subvector.sort_unstable_by_key(|edge| Reverse(nodes[edge.to].rank));
            }

            // get permutation
            let new_edge_index = get_indices(&data.edges);

            // clear index
            clear_indices(&mut data.edges);

            // sort costs according to permutation
            fix_cost(&mut data.edge_costs, &new_edge_index, dim);

            // resolve pch-indices
            fix_contracted_edges(&mut data.edges, &new_edge_index);

            // down_index update
            let edges = &data.edges;
            for node in 0..data.nodes.len() {
                let subvector =
                    &mut data.down_index[data.down_offset[node]..data.down_offset[node + 1]];
                subvector
                    .iter_mut()
                    .for_each(|edge_id| *edge_id = new_edge_index[*edge_id]);
                subvector.sort_unstable_by_key(|edge_id| Reverse(nodes[edges[*edge_id].from].rank));
            }
        }
        QueryType::Pcrp => {
            // add id to keep track of permuation
            set_indices(&mut data.edges);

            // sort by level
            for node in 0..data.nodes.len() {
                let subvector = &mut data.edges[data.up_offset[node]..data.up_offset[node + 1]];
                subvector.sort_unstable_by_key(|edge| Reverse(edge.level));
            }

            // get permutation
            let new_edge_index = get_indices(&data.edges);

            // clear index
            clear_indices(&mut data.edges);

            // sort costs according to permutation
            fix_cost(&mut data.edge_costs, &new_edge_index, dim);

            // resolve pch-indices
            fix_contracted_edges(&mut data.edges, &new_edge_index);

            // down_index update
            let edges = &data.edges;
            for node in 0..data.nodes.len() {
                let subvector =
                    &mut data.down_index[data.down_offset[node]..data.down_offset[node + 1]];
                subvector
                    .iter_mut()
                    .for_each(|edge_id| *edge_id = new_edge_index[*edge_id]);
                subvector.sort_unstable_by_key(|edge_id| Reverse(edges[*edge_id].level));
            }
        }
        QueryType::Prp => {
            // add id to keep track of permuation
            set_indices(&mut data.edges);

            let nodes = &data.nodes;
            let minimum_rank_of_layer_one = data
                .edges
                .iter()
                .map(|edge| {
                    if edge.level == Some(1) {
                        nodes[edge.from].rank
                    } else {
                        usize::MAX
                    }
                })
                .min()
                .unwrap();

            for node in 0..data.nodes.len() {
                let subvector = &mut data.edges[data.up_offset[node]..data.up_offset[node + 1]];
                // sort by level then by rank but in wrong order
                subvector.sort_unstable_by(|a, b| {
                    a.level
                        .cmp(&b.level)
                        .reverse()
                        .then(nodes[a.from].rank.cmp(&nodes[b.from].rank))
                });
                // find index of pch edges
                let pch_level_one_index = subvector
                    .iter()
                    .enumerate()
                    .find(|(_index, &edge)| {
                        if edge.level.is_none() {
                            nodes[edge.from].rank > minimum_rank_of_layer_one
                        } else {
                            false
                        }
                    })
                    .map(|(index, _edge)| index)
                    .unwrap_or(subvector.len());

                // split by index
                let new_subvector = &mut subvector[..pch_level_one_index];

                // sort new subvector by pch-rank
                new_subvector.sort_unstable_by_key(|edge| Reverse(nodes[edge.to].rank));
            }

            // get permutation
            let new_edge_index = get_indices(&data.edges);

            // clear index
            clear_indices(&mut data.edges);

            // sort costs according to permutation
            fix_cost(&mut data.edge_costs, &new_edge_index, dim);

            // resolve pch-indices
            fix_contracted_edges(&mut data.edges, &new_edge_index);

            // down_index update
            let edges = &data.edges;
            for node in 0..data.nodes.len() {
                let subvector =
                    &mut data.down_index[data.down_offset[node]..data.down_offset[node + 1]];
                subvector
                    .iter_mut()
                    .for_each(|edge_id| *edge_id = new_edge_index[*edge_id]);

                // sort by level then by rank but in wrong order
                subvector.sort_unstable_by(|a, b| {
                    edges[*a]
                        .level
                        .cmp(&edges[*b].level)
                        .reverse()
                        .then(nodes[edges[*a].to].rank.cmp(&nodes[edges[*b].to].rank))
                });
                // find index of pch edges
                let pch_level_one_index = subvector
                    .iter()
                    .enumerate()
                    .find(|(_index, &edge)| {
                        if edges[edge].level.is_none() {
                            nodes[edges[edge].to].rank > minimum_rank_of_layer_one
                        } else {
                            false
                        }
                    })
                    .map(|(index, _edge)| index)
                    .unwrap_or(subvector.len());

                // split by index
                let new_subvector = &mut subvector[..pch_level_one_index];

                // sort new subvector by pch-rank
                new_subvector.sort_unstable_by_key(|edge| Reverse(nodes[edges[*edge].from].rank));
            }
        }
    }
}

fn fix_cost(edge_costs: &mut Vec<Cost>, new_edge_index: &[EdgeId], dim: usize) {
    for (new_index, edge_cost) in new_edge_index
        .iter()
        .zip(edge_costs.clone().chunks_exact(dim))
    {
        for i in 0..dim {
            edge_costs[(new_index * dim) + i] = edge_cost[i];
        }
    }
}

fn fix_contracted_edges(edges: &mut [Edge], new_edge_index: &[EdgeId]) {
    edges.par_iter_mut().for_each(|edge| {
        if edge.contracted_edges.is_some() {
            edge.contracted_edges = Some((
                new_edge_index[edge.contracted_edges.unwrap().0],
                new_edge_index[edge.contracted_edges.unwrap().1],
            ));
        }
    });
}

fn set_indices(edges: &mut [Edge]) {
    edges
        .par_iter_mut()
        .enumerate()
        .for_each(|(i, edge)| edge.old_id = Some(i));
}

fn get_indices(edges: &[Edge]) -> Vec<EdgeId> {
    let mut new_edge_index = vec![INVALID_NODE; edges.len()];
    edges
        .iter()
        .enumerate()
        .for_each(|(i, node)| new_edge_index[node.old_id.unwrap()] = i);
    new_edge_index
}

fn clear_indices(edges: &mut [Edge]) {
    edges.par_iter_mut().for_each(|edge| edge.old_id = None);
}

use super::*;

pub fn create_bidirect(edges: &[Edge]) -> Vec<Edge> {
    // have edges in both directions
    let mut new_edges = Vec::with_capacity(edges.len());
    for edge in edges {
        new_edges.push(Edge::new(
            edge.from,
            edge.to,
            edge.cost.clone(),
            edge.contracted_edges,
        ));
        new_edges.push(Edge::new(
            edge.to,
            edge.from,
            edge.cost.clone(),
            edge.contracted_edges,
        ));
    }

    // now remove duplicates them
    new_edges.par_sort_unstable();
    new_edges.dedup();
    new_edges
}

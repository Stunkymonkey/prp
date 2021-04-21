use super::*;

pub fn create_bidirect(edges: &mut Vec<Edge>) {
    // have edges in both directions
    let mut new_edges = Vec::with_capacity(edges.len());
    for edge in edges.clone() {
        new_edges.push(Edge::new(edge.to, edge.from));
    }
    edges.extend(new_edges);

    // now remove duplicates them
    edges.par_sort_unstable();
    edges.dedup();
}

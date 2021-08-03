use super::*;
use query_export::Export;

pub mod bidirectional;
pub mod normal;
pub mod pch;
pub mod pcrp;
pub mod prp;

pub trait FindPath<E: Export> {
    fn new(amount_nodes: usize, exporter: E) -> Self
    where
        Self: Sized;
    fn reset_state(&mut self);
    fn get_query_export(&self) -> &E;
    fn find_path(
        &mut self,
        from: NodeId,
        to: NodeId,
        alpha: Vec<f64>,
        graph: &Graph,
        nodes: &[Node],
        _mlp_levels: &[usize],
    ) -> Option<(Vec<NodeId>, Cost)>;
}

pub fn get<E: 'static + Export>(
    query_type: QueryType,
    amount_nodes: usize,
    exporter: E,
) -> Box<dyn FindPath<E>> {
    match query_type {
        QueryType::Normal => Box::new(dijkstra::normal::Dijkstra::new(amount_nodes, exporter)),
        QueryType::Bi => Box::new(dijkstra::bidirectional::Dijkstra::new(
            amount_nodes,
            exporter,
        )),
        QueryType::Pch => Box::new(dijkstra::pch::Dijkstra::new(amount_nodes, exporter)),
        QueryType::Pcrp => Box::new(dijkstra::pcrp::Dijkstra::new(amount_nodes, exporter)),
        QueryType::Prp => Box::new(dijkstra::prp::Dijkstra::new(amount_nodes, exporter)),
    }
}

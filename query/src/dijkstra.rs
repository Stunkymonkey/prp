use super::*;
use query_export::Export;

pub mod bidirectional;
pub mod crp;
pub mod normal;
pub mod pch;
pub mod prp;

pub trait FindPath<E: Export> {
    fn new(amount_nodes: usize, exporter: E) -> Self
    where
        Self: Sized;
    fn reset_state(&mut self);
    fn find_path(
        &mut self,
        from: NodeId,
        to: NodeId,
        alpha: Vec<f64>,
        graph: &Graph,
        nodes: &[Node],
        _mlp_layers: &[usize],
    ) -> Option<(Vec<NodeId>, Cost)>;
}

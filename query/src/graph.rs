use super::*;

#[derive(Clone, Debug)]
pub struct Graph {
    pub edges: Vec<Edge>,
    pub edge_costs: Vec<Cost>,
    pub up_offset: Vec<EdgeId>,
    pub down_offset: Vec<EdgeId>,
    pub down_index: Vec<EdgeId>,
    dim: usize,
}

impl Graph {
    pub fn new(
        edges: Vec<Edge>,
        edge_costs: Vec<Cost>,
        up_offset: Vec<EdgeId>,
        down_offset: Vec<EdgeId>,
        down_index: Vec<EdgeId>,
        dim: usize,
    ) -> Self {
        Graph {
            edges,
            edge_costs,
            up_offset,
            down_offset,
            down_index,
            dim,
        }
    }

    /// get single edge
    #[allow(dead_code)]
    pub fn get_edge(&self, id: EdgeId) -> Edge {
        self.edges[id]
    }

    /// get single edge costs
    #[allow(dead_code)]
    pub fn get_edge_costs(&self, id: EdgeId) -> &[Cost] {
        let offset = self.dim * id;
        &self.edge_costs[offset..offset + self.dim]
    }

    /// get all up edges from one node
    #[allow(dead_code)]
    pub fn get_edges_from_id(&self, ids: Vec<EdgeId>) -> Vec<Edge> {
        ids.iter().map(|x| self.edges[*x]).collect()
    }

    /// get all up edge-ids from one node
    #[allow(dead_code)]
    pub fn get_up_edge_ids(&self, node: NodeId) -> Vec<EdgeId> {
        (self.up_offset[node]..self.up_offset[node + 1]).collect()
    }

    /// get all down edge-ids from one node
    #[allow(dead_code)]
    pub fn get_down_edge_ids(&self, node: NodeId) -> Vec<EdgeId> {
        let prev: Vec<EdgeId> = (self.down_offset[node]..self.down_offset[node + 1]).collect();
        prev.iter().map(|x| self.down_index[*x]).collect()
    }

    /// get all edge-ids from one node
    #[allow(dead_code)]
    pub fn get_edge_ids(&self, node: NodeId) -> (Vec<EdgeId>, Vec<EdgeId>) {
        let outgoing: Vec<NodeId> = self.get_up_edge_ids(node);
        let incomming: Vec<NodeId> = self.get_down_edge_ids(node);
        (outgoing, incomming)
    }

    /// get all edge-ids from one node
    #[allow(dead_code)]
    pub fn get_all_edge_ids(&self, node: NodeId) -> Vec<EdgeId> {
        let (outgoing, incomming) = self.get_edge_ids(node);
        let mut connected_edges = outgoing;
        connected_edges.extend(&incomming);
        connected_edges
    }

    /// get all up neighbors from one node
    #[allow(dead_code)]
    pub fn get_up_neighbors(&self, node: NodeId) -> Vec<EdgeId> {
        let next = self.get_up_edge_ids(node);
        let mut tmp: Vec<EdgeId> = next.iter().map(|x| self.edges[*x].to).collect();
        tmp.dedup();
        tmp
    }

    /// get all up neighbors from one node
    #[allow(dead_code)]
    pub fn get_down_neighbors(&self, node: NodeId) -> Vec<EdgeId> {
        let prev = self.get_down_edge_ids(node);
        let mut tmp: Vec<EdgeId> = prev.iter().map(|x| self.edges[*x].from).collect();
        tmp.par_sort_unstable();
        tmp.dedup();
        tmp
    }

    /// returning all previous and next neighbors
    #[allow(dead_code)]
    pub fn get_neighbours(&self, node: NodeId) -> (Vec<usize>, Vec<usize>) {
        let targets: Vec<NodeId> = self.get_up_neighbors(node);
        let sources: Vec<NodeId> = self.get_down_neighbors(node);
        (targets, sources)
    }

    /// returning all neighbors
    #[allow(dead_code)]
    pub fn get_all_neighbours(&self, node: NodeId) -> Vec<usize> {
        let (targets, sources) = self.get_neighbours(node);
        let mut neighbours = targets;
        neighbours.extend(&sources);
        neighbours.par_sort_unstable();
        neighbours.dedup();
        neighbours
    }

    /// amount of neighbors
    #[allow(dead_code)]
    pub fn node_degree(&self, node: NodeId) -> usize {
        self.up_offset[node + 1] - self.up_offset[node] + self.down_offset[node + 1]
            - self.down_offset[node]
    }
}

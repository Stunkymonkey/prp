use super::*;

use std::collections::BTreeSet;

pub trait Export {
    fn new() -> Self
    where
        Self: Sized;
    fn reset(&mut self);
    fn heap_pop(&mut self);
    fn visited_node(&mut self, _node_id: NodeId);
    fn visited_edge(&mut self, _edge_id: Option<EdgeId>);
    fn relaxed_edge(&mut self);
    fn visited_partition(&mut self, _layer_height: LayerHeight, _partition_id: PartitionId);
    fn current_meeting_point(&mut self, _node_id: NodeId);
}

pub struct NoOp;
impl Export for NoOp {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {}
    }
    fn reset(&mut self) {}
    fn heap_pop(&mut self) {}
    fn visited_node(&mut self, _node_id: NodeId) {}
    fn visited_edge(&mut self, _edge_id: Option<EdgeId>) {}
    fn relaxed_edge(&mut self) {}
    fn visited_partition(&mut self, _layer_height: LayerHeight, _partition_id: PartitionId) {}
    fn current_meeting_point(&mut self, _node_id: NodeId) {}
}

#[derive(Debug)]
pub struct Counter {
    pub heap_pops: usize,
    pub relaxed_edges: usize,
}
impl Export for Counter {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {
            heap_pops: 0,
            relaxed_edges: 0,
        }
    }
    fn reset(&mut self) {
        self.heap_pops = 0;
        self.relaxed_edges = 0;
    }
    fn heap_pop(&mut self) {
        self.heap_pops += 1;
    }
    fn visited_node(&mut self, _node_id: NodeId) {}
    fn visited_edge(&mut self, _edge_id: Option<EdgeId>) {}
    fn relaxed_edge(&mut self) {
        self.relaxed_edges += 1;
    }
    fn visited_partition(&mut self, _layer_height: LayerHeight, _partition_id: PartitionId) {}
    fn current_meeting_point(&mut self, _node_id: NodeId) {}
}

#[derive(Debug)]
pub struct RealExport {
    pub heap_pops: usize,
    pub visited_nodes: Vec<NodeId>,
    pub visited_edges: Vec<EdgeId>,
    pub relaxed_edges: usize,
    pub visited_partitions: BTreeSet<(LayerHeight, PartitionId)>,
    pub meeting_node: NodeId,
}
impl Export for RealExport {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {
            heap_pops: 0,
            visited_nodes: Vec::new(),
            visited_edges: Vec::new(),
            relaxed_edges: 0,
            visited_partitions: BTreeSet::new(),
            meeting_node: INVALID_NODE,
        }
    }
    fn reset(&mut self) {
        self.heap_pops = 0;
        self.visited_nodes.clear();
        self.visited_edges.clear();
        self.relaxed_edges = 0;
        self.visited_partitions.clear();
        self.meeting_node = INVALID_NODE;
    }
    fn heap_pop(&mut self) {
        self.heap_pops += 1;
    }
    fn visited_node(&mut self, node_id: NodeId) {
        self.visited_nodes.push(node_id);
    }
    fn visited_edge(&mut self, prev_edge: Option<EdgeId>) {
        if let Some(edge_id) = prev_edge {
            self.visited_edges.push(edge_id);
        }
    }
    fn relaxed_edge(&mut self) {
        self.relaxed_edges += 1;
    }
    fn visited_partition(&mut self, layer_height: LayerHeight, partition_id: PartitionId) {
        self.visited_partitions.insert((layer_height, partition_id));
    }

    fn current_meeting_point(&mut self, node_id: NodeId) {
        self.meeting_node = node_id;
    }
}

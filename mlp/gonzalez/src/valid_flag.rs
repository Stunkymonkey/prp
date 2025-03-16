use crate::constants::*;

#[derive(Clone)]
pub struct ValidFlag {
    // maybe only use a u16/u32 for less memory consumption?
    nodes: Vec<usize>,
    valid_flag: usize,
}

impl ValidFlag {
    pub fn new(num_nodes: usize) -> Self {
        ValidFlag {
            nodes: vec![0; num_nodes],
            valid_flag: 1,
        }
    }

    pub fn is_valid(&self, node: NodeId) -> bool {
        self.nodes[node] == self.valid_flag
    }

    pub fn set_valid(&mut self, node: NodeId) {
        self.nodes[node] = self.valid_flag;
    }

    pub fn invalidate_all(&mut self) {
        if self.valid_flag == usize::MAX {
            self.nodes = vec![0; self.nodes.len()];
            self.valid_flag = 1;
        } else {
            self.valid_flag += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_valid() {
        let mut visited = ValidFlag::new(42);
        assert!(!visited.is_valid(17));
        visited.set_valid(17);
        assert!(visited.is_valid(17));
        visited.invalidate_all();
        assert!(!visited.is_valid(17));
    }
}

use std::fs::File;
use std::io::{BufWriter, Write};

use crate::structs::*;

pub fn write_mlp(mlp_file: &str, partitions: &[usize], nodes: &[Node]) -> std::io::Result<()> {
    let f = File::create(mlp_file)?;
    let mut f = BufWriter::new(f);
    // layers
    writeln!(f, "{}", partitions.len())?;
    for partition in partitions {
        writeln!(f, "{}", partition)?;
    }
    // nodes
    writeln!(f, "{}", nodes.len())?;
    for node in nodes {
        writeln!(f, "{}", node.partition)?;
    }
    f.flush()?;
    Ok(())
}

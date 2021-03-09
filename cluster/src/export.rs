use std::fs::File;
use std::io::{BufWriter, Write};

use crate::structs::*;

pub fn write_mlp(mlp_file: &str, clusters: &[usize], nodes: &[Node]) -> std::io::Result<()> {
    let f = File::create(mlp_file)?;
    let mut f = BufWriter::new(f);
    // layers
    writeln!(f, "{}", clusters.len())?;
    for cluster in clusters {
        writeln!(f, "{}", cluster)?;
    }
    // nodes
    writeln!(f, "{}", nodes.len())?;
    for node in nodes {
        writeln!(f, "{}", node.cluster)?;
    }
    f.flush()?;
    Ok(())
}

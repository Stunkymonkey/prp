use super::*;

use std::fs::File;
use std::io::{BufWriter, Write};

pub fn write_file(
    file_path: &str,
    header: &str,
    nodes: &[Node],
    edges: &[Edge],
) -> std::io::Result<()> {
    let f = File::create(file_path)?;
    let mut f = BufWriter::new(f);

    f.write_all(header.as_bytes())?;
    f.write_all("\n".as_bytes())?;

    f.write_all((edges[0].cost.len().to_string() + "\n").as_bytes())?;
    f.write_all((nodes.len().to_string() + "\n").as_bytes())?;
    f.write_all((edges.len().to_string() + "\n").as_bytes())?;

    for (node_id, node) in nodes.iter().enumerate() {
        f.write_all(
            format!(
                "{:?} {:?} {} {} {} 0\n",
                node_id, node.osm_id, node.latitude, node.longitude, node.height
            )
            .as_bytes(),
        )?
    }

    for edge in edges {
        let mut costs = "".to_string();
        for cost in &edge.cost {
            costs.push_str(&format!("{} ", cost));
        }
        f.write_all(
            format!(
                // space for costs is ommited, because it is added in the costs string before
                "{:?} {:?} {}{} {}\n",
                edge.from, edge.to, costs, edge.contracted_edges.0, edge.contracted_edges.1
            )
            .as_bytes(),
        )?
    }

    Ok(())
}

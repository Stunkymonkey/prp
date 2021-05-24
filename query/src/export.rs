use super::*;

use std::fs::File;
use std::io::{BufWriter, Write};

pub fn write_file(file_path: &str, output: &str) -> std::io::Result<()> {
    let mut file = File::create(file_path)?;
    file.write_all(output.as_bytes())?;
    Ok(())
}

pub fn write_wkt_file(
    file_path: &str,
    from: NodeId,
    to: NodeId,
    meeting: Option<NodeId>,
    visited_nodes: &[NodeId],
    path: &[NodeId],
    visited_edges: &[EdgeId],
    nodes: &[Node],
    edges: &[Edge],
) -> std::io::Result<()> {
    let f = File::create(file_path.replace(".wkt", "-query.wkt"))?;
    let mut f = BufWriter::new(f);
    f.write_all("WKT-POINTS; type\n".as_bytes())?;

    let line = format!(
        "POINT ({:?} {:?}); 0\n",
        nodes[from].longitude, nodes[from].latitude
    );
    f.write_all(line.as_bytes())?;
    let line = format!(
        "POINT ({:?} {:?}); 0\n",
        nodes[to].longitude, nodes[to].latitude
    );
    f.write_all(line.as_bytes())?;
    if let Some(meeting_node) = meeting {
        let line = format!(
            "POINT ({:?} {:?}); 1\n",
            nodes[meeting_node].longitude, nodes[meeting_node].latitude
        );
        f.write_all(line.as_bytes())?;
    }

    let f = File::create(file_path.replace(".wkt", "-nodes.wkt"))?;
    let mut f = BufWriter::new(f);
    f.write_all("WKT-POINTS; layer_height\n".as_bytes())?;

    for node_id in visited_nodes {
        let line = format!(
            "POINT ({:?} {:?}); {:?}\n",
            nodes[*node_id].longitude, nodes[*node_id].latitude, nodes[*node_id].layer_height
        );
        f.write_all(line.as_bytes())?;
    }

    let f = File::create(file_path.replace(".wkt", "-path.wkt"))?;
    let mut f = BufWriter::new(f);
    f.write_all("WKT-LINESTRINGS\n".as_bytes())?;

    for (prev, next) in path.iter().zip(path.iter().skip(1)) {
        let line = format!(
            "LINESTRING ({:?} {:?}, {:?} {:?})\n",
            nodes[*prev].longitude,
            nodes[*prev].latitude,
            nodes[*next].longitude,
            nodes[*next].latitude,
        );
        f.write_all(line.as_bytes())?;
    }

    let f = File::create(file_path.replace(".wkt", "-edges.wkt"))?;
    let mut f = BufWriter::new(f);
    f.write_all("WKT-LINESTRINGS\n".as_bytes())?;

    for edge_id in visited_edges {
        let line = format!(
            "LINESTRING ({:?} {:?}, {:?} {:?})\n",
            nodes[edges[*edge_id].from].longitude,
            nodes[edges[*edge_id].from].latitude,
            nodes[edges[*edge_id].to].longitude,
            nodes[edges[*edge_id].to].latitude,
        );
        f.write_all(line.as_bytes())?;
    }

    Ok(())
}

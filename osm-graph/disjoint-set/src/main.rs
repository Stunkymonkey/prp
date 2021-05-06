mod arguments;
mod bidirect_graph;
mod constants;
mod disjoint_set;
mod fmi_export;
mod fmi_import;
mod offset;
mod structs;

use constants::*;
use structs::*;

use rayon::prelude::*;
use std::collections::BTreeMap;

fn main() {
    // check/get arguments
    let (in_file, out_file): (String, String) = match arguments::get_arguments() {
        Ok(result) => result,
        Err(error) => panic!("error while parsing arguments: {:?}", error),
    };

    let mut nodes = Vec::<Node>::new();
    let mut edges = Vec::<Edge>::new();

    // read pbfextrator-file
    let header = match fmi_import::read_file(&in_file, &mut nodes, &mut edges) {
        Ok(result) => {
            println!("reading files finished");
            result
        }
        Err(error) => panic!("error while reading file: {:?}", error),
    };

    // make graph bidirect
    let bi_edges = bidirect_graph::create_bidirect(&edges);

    // extract all node-ids wanted
    let keeping_nodes = disjoint_set::get_disjoint_nodes(&nodes, &bi_edges);

    println!(
        "largest_set contains {:?} nodes of original {:?} nodes -> keeping {:.3}%",
        keeping_nodes.len(),
        nodes.len(),
        (keeping_nodes.len() as f64 / nodes.len() as f64) * 100.0
    );

    // get new ids
    let mut new_node_ids = BTreeMap::new();
    for (new_node_id, old_node_id) in keeping_nodes.iter().enumerate() {
        new_node_ids.insert(old_node_id, new_node_id);
    }

    let mut resulting_edges = Vec::with_capacity(edges.len());

    for edge in &edges {
        if new_node_ids.contains_key(&edge.from) {
            let from = *new_node_ids.get(&edge.from).unwrap();
            let to = *new_node_ids.get(&edge.to).unwrap();
            resulting_edges.push(Edge::new(
                from,
                to,
                edge.cost.clone(),
                edge.contracted_edges,
            ));
        }
    }

    let mut resulting_nodes = Vec::with_capacity(nodes.len());
    for node_id in keeping_nodes {
        resulting_nodes.push(nodes[node_id].clone());
    }

    println!(
        "largest_set contains {:?} edges of original {:?} edges -> keeping {:.3}%",
        resulting_edges.len(),
        edges.len(),
        (resulting_edges.len() as f64 / edges.len() as f64) * 100.0
    );

    // write export file
    match fmi_export::write_file(&out_file, &header, &resulting_nodes, &resulting_edges) {
        Ok(_result) => println!("fmi exported"),
        Err(error) => panic!("error while writing file: {:?}", error),
    };
}

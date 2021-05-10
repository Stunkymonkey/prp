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

fn main() {
    // check/get arguments
    let (in_file, out_file): (String, String) = match arguments::get_arguments() {
        Ok(result) => result,
        Err(error) => panic!("error while parsing arguments: {:?}", error),
    };

    let mut nodes = Vec::<Node>::new();
    let mut edges = Vec::<Edge>::new();

    // read pbfextrator-file
    let mut header = match fmi_import::read_file(&in_file, &mut nodes, &mut edges) {
        Ok(result) => {
            println!("reading files finished");
            result
        }
        Err(error) => panic!("error while reading file: {:?}", error),
    };

    // add note that the file was modified, to distinct them
    header = header.replace(
        "# Build by: pbfextractor",
        "# Build by: pbfextractor\n# Modified by: fmi-disjoint-set",
    );

    // make graph bidirect
    let bi_edges = bidirect_graph::create_bidirect(&edges);

    // extract all node-ids wanted
    let keeping_nodes = disjoint_set::get_largest_disjoint_set(&nodes, &bi_edges);

    println!(
        "largest_set contains {:?} nodes of original {:?} nodes -> keeping {:.3}%",
        keeping_nodes.len(),
        nodes.len(),
        (keeping_nodes.len() as f64 / nodes.len() as f64) * 100.0
    );

    // remove unwanted nodes/edges
    let (resulting_nodes, resulting_edges) =
        disjoint_set::keep_only_nodes_from_set(&keeping_nodes, &nodes, &edges);

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

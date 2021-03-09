#[macro_use]
extern crate clap;

mod arguments;
mod constants;
mod export;
mod fmi_import;
mod partition;
mod structs;

use constants::*;
use structs::*;

fn main() {
    // check/get arguments
    let (fmi_file, mlp_file, clusters): (String, String, Vec<usize>) =
        match arguments::get_arguments() {
            Ok(result) => result,
            Err(error) => panic!("error while parsing arguments: {:?}", error),
        };

    let mut nodes = Vec::<Node>::new();
    let mut edges = Vec::<Edge>::new();

    // read pbfextrator-file
    match fmi_import::read_file(&fmi_file, &mut nodes, &mut edges) {
        Ok(_result) => println!("reading files finished"),
        Err(error) => panic!("error while reading file: {:?}", error),
    };

    // check that the amount of nodes is not smaller, then then amount of clusters
    assert!(
        nodes.len() > clusters.iter().product::<usize>(),
        "amount of clusters to high or nodes to small"
    );

    // do partitioning
    match partition::partition(&clusters, &mut nodes) {
        Ok(_result) => println!("creating partitions sucessfully"),
        Err(error) => panic!("error while creating partitions: {:?}", error),
    };

    // check if all nodes have a valid cluster
    for node in &nodes {
        assert!(
            node.cluster != INVALID_CLUSTER,
            "at least one node has not been assigned to any cluster"
        );
    }

    // write export file
    match export::write_mlp(&mlp_file, &clusters, &nodes) {
        Ok(_result) => println!("mlp exported"),
        Err(error) => panic!("error while writing file: {:?}", error),
    };
}

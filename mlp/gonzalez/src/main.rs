mod arguments;
mod bidirect_graph;
mod constants;
mod export;
mod fmi_import;
mod gonzalez;
mod graph_helper;
mod hop_bfs;
mod offset;
mod structs;
mod valid_flag;

use constants::*;
use structs::*;

use rayon::prelude::*;
use std::time::Instant;

fn main() {
    // check/get arguments
    let (fmi_file, mlp_file, partition_amount): (String, String, Vec<usize>) =
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

    // check that the amount of nodes is not smaller, then then amount of partitions
    assert!(
        nodes.len() > *partition_amount.iter().max().unwrap_or(&0),
        "amount of partitions to high or nodes to small"
    );

    // make graph bidirect
    bidirect_graph::create_bidirect(&mut edges);

    // do partitioning
    let partition_time = Instant::now();
    match gonzalez::partition(&partition_amount, &mut nodes, &mut edges) {
        Ok(_result) => println!("creating partitions sucessfully"),
        Err(error) => panic!("error while creating partitions: {:?}", error),
    };
    println!("MLP in: {:?}", partition_time.elapsed());

    // check if all nodes have a valid cluster
    for node in &nodes {
        assert!(
            node.partition != INVALID_PARTITION,
            "at least one node has not been assigned to any cluster"
        );
    }

    // write export file
    match export::write_mlp(&mlp_file, &partition_amount, &nodes) {
        Ok(_result) => println!("mlp exported"),
        Err(error) => panic!("error while writing file: {:?}", error),
    };
}

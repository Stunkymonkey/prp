mod arguments;
mod constants;
mod export;
mod fmi_import;
mod partition;
mod structs;

use constants::*;
use structs::*;

use std::time::Instant;

fn main() {
    // check/get arguments
    let (fmi_file, mlp_file, partitions): (String, String, Vec<usize>) =
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
        nodes.len() > partitions.iter().product::<usize>(),
        "amount of partitions to high or nodes to small"
    );

    // do partitioning
    let partition_time = Instant::now();
    match partition::partition(&partitions, &mut nodes) {
        Ok(_result) => println!("creating partitions sucessfully"),
        Err(error) => panic!("error while creating partitions: {:?}", error),
    };
    println!("MLP-Layer in: {:?}", partition_time.elapsed());

    // check if all nodes have a valid partition
    for node in &nodes {
        assert!(
            node.partition != INVALID_PARTITION,
            "at least one node has not been assigned to any partition"
        );
    }

    // write export file
    match export::write_mlp(&mlp_file, &partitions, &nodes) {
        Ok(_result) => println!("mlp exported"),
        Err(error) => panic!("error while writing file: {:?}", error),
    };
}

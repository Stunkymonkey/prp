mod arguments;
mod constants;
mod contraction;
mod export;
mod fmi_import;
mod graph_helper;
mod grid;
mod min_heap;
mod mlp_helper;
mod mlp_import;
mod ndijkstra;
mod offset;
mod ordering;
mod structs;
mod valid_flag;

use constants::*;
use structs::*;

use rayon::prelude::*;
use std::time::Instant;

fn main() {
    let (fmi_file, mlp_file, contraction_stop, output_file) = match arguments::get_arguments() {
        Ok(result) => result,
        Err(error) => panic!("error while parsing arguments: {:?}", error),
    };
    let mut mlp_levels = Vec::<usize>::new();
    let mut nodes = Vec::<Node>::new();
    let mut edges = Vec::<Edge>::new();
    let mut metrics = Vec::<String>::new();

    match fmi_import::read_file(&fmi_file, &mut nodes, &mut edges, &mut metrics) {
        Ok(_result) => println!("reading pbfextractor file finished"),
        Err(error) => panic!("error while reading pbfextractor file: {:?}", error),
    };
    // if mlp-file is not provided all nodes will go into one partition
    match mlp_file.as_str() {
        "" => {
            nodes.iter_mut().for_each(|node| node.partition = 0);
            mlp_levels = vec![1];
        }
        _ => {
            match mlp_import::read_file(&mlp_file, &mut nodes, &mut mlp_levels) {
                Ok(_result) => println!("reading mlp file finished"),
                Err(error) => panic!("error while reading mlp file: {:?}", error),
            };
        }
    }

    let mut up_offset = Vec::<EdgeId>::new();
    let mut down_offset = Vec::<EdgeId>::new();

    let mut down_index =
        offset::generate_offsets(&mut edges, &mut up_offset, &mut down_offset, nodes.len());

    let highest_diff_time = Instant::now();
    mlp_helper::calculate_levels(
        &mut nodes,
        &edges,
        &up_offset,
        &down_offset,
        &down_index,
        &mlp_levels,
    );
    println!("MLP in: {:?}", highest_diff_time.elapsed());

    let contraction_time = Instant::now();
    contraction::prp_contraction(
        &mut nodes,
        &mut edges,
        &mut up_offset,
        &mut down_offset,
        &mut down_index,
        &mlp_levels,
        contraction_stop,
    );
    println!("Contraction in: {:?}", contraction_time.elapsed());

    let edge_costs: Vec<Cost> = edges.iter().map(|e| e.cost.clone()).flatten().collect();

    let grid_time = Instant::now();
    let mut grid_offset = Vec::<GridId>::new();
    let mut grid = Vec::<NodeId>::new();

    let grid_bounds = grid::generate_grid(&mut grid, &mut grid_offset, &nodes);
    println!("Generate grid in: {:?}", grid_time.elapsed());

    let result = BinFile {
        nodes,
        mlp_levels,
        edges,
        edge_costs,
        up_offset,
        down_offset,
        down_index,
        grid_offset,
        grid,
        grid_bounds,
        metrics,
    };

    match export::write_to_disk(&output_file, &result) {
        Ok(_result) => println!("writing bin file finished"),
        Err(error) => panic!("error while writing bin file: {:?}", error),
    };
}

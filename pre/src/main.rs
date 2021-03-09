mod arguments;
mod constants;
mod fmi_import;
mod mlp_import;
mod structs;

use constants::*;
use structs::*;

fn main() {
    let (fmi_file, mlp_file, _output_file) = match arguments::get_arguments() {
        Ok(result) => result,
        Err(error) => panic!("error while parsing arguments: {:?}", error),
    };
    let mut mlp_layers = Vec::<usize>::new();
    let mut nodes = Vec::<Node>::new();
    let mut edges = Vec::<Edge>::new();
    let mut edge_weights = Vec::<Weight>::new();

    match fmi_import::read_file(&fmi_file, &mut nodes, &mut edges, &mut edge_weights) {
        Ok(_result) => println!("reading pbfextractor file finished"),
        Err(error) => panic!("error while reading pbfextractor file: {:?}", error),
    };
    match mlp_import::read_file(&mlp_file, &mut nodes, &mut mlp_layers) {
        Ok(_result) => println!("reading mlp file finished"),
        Err(error) => panic!("error while reading mlp file: {:?}", error),
    };

    println!("edge len {:?}", edges.len());
    println!("edge_weights len {:?}", edge_weights.len());
}

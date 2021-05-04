// use rayon::prelude::*;
use std::time::Instant;

use prp_query::*;

fn main() {
    let (fmi_file, eval_file) = get_arguments();
    // read binfile
    let data: BinFile = match bin_import::read_file(&fmi_file) {
        Ok(result) => result,
        Err(error) => panic!("error while reading bin-file: {:?}", error),
    };

    //read csvfile
    let eval: Vec<Vec<String>> = match json_import::read_file(&eval_file) {
        Ok(result) => result,
        Err(error) => panic!("error while reading eval-file: {:?}", error),
    };

    let amount_nodes = data.nodes.len();
    let dim = data.edge_costs.len() / data.edges.len();

    let graph = Graph::new(
        data.edges,
        data.edge_costs,
        data.up_offset,
        data.down_offset,
        data.down_index,
        dim,
    );
    let data = WebData {
        nodes: data.nodes,
        graph,
        grid_offset: data.grid_offset,
        grid: data.grid,
        grid_bounds: data.grid_bounds,
        metrics: data.metrics,
    };

    println!("eval-lines {:?}", eval.len());

    let start_id = 100;
    let end_id = 77;
    let alpha = vec![0.1, 0.9];

    let mut dijkstra = Dijkstra::new(amount_nodes, dim);

    let dijkstra_time = Instant::now();
    let _tmp = dijkstra.find_path(start_id, end_id, alpha, &data.graph, &data.nodes);
    println!("    Dijkstra in: {:?}", dijkstra_time.elapsed());
}

fn get_arguments() -> (String, String) {
    let matches = clap::App::new("prp_eval")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about("provides webinterface and testing option")
        .arg(
            clap::Arg::with_name("fmi-file")
                .help("the input file to use")
                .takes_value(true)
                .short("f")
                .long("file")
                .required(true),
        )
        .arg(
            clap::Arg::with_name("eval-file")
                .help("the CSV file it will evaluate")
                .takes_value(true)
                .short("e")
                .long("eval-file")
                .required(true),
        )
        .get_matches();

    (
        matches.value_of("fmi-file").unwrap().to_string(),
        matches.value_of("eval-file").unwrap().to_string(),
    )
}

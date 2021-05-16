// use rayon::prelude::*;
use std::fs::File;
use std::time::Instant;

use prp_query::dijkstra_export::*;
use prp_query::*;

fn main() {
    let (fmi_file, eval_file, export, export_path) = get_arguments();
    // read binfile
    let data: BinFile = match bin_import::read_file(&fmi_file) {
        Ok(result) => result,
        Err(error) => panic!("error while reading bin-file: {:?}", error),
    };

    //read eval-file
    let file = File::open(eval_file).expect("file should open read only");
    let mut eval: Vec<EvalPoint> =
        serde_json::from_reader(file).expect("file should be proper JSON");

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

    println!("amount of evaluation-points: {:?}", eval.len());

    for query in eval.iter_mut() {
        if query.start_id.is_none() {
            let start = Location {
                latitude: query.start_pos.latitude,
                longitude: query.start_pos.longitude,
            };
            query.start_id = Some(grid::get_closest_point(
                start,
                &data.nodes,
                &data.grid,
                &data.grid_offset,
                &data.grid_bounds,
            ));
        }
        if query.end_id.is_none() {
            let end = Location {
                latitude: query.end_pos.latitude,
                longitude: query.end_pos.longitude,
            };
            query.end_id = Some(grid::get_closest_point(
                end,
                &data.nodes,
                &data.grid,
                &data.grid_offset,
                &data.grid_bounds,
            ));
        }
    }

    println!("calculated all closest-point node_ids");

    if !export {
        let mut dijkstra = Dijkstra::new(amount_nodes, dim, NoOp::new());

        let dijkstra_time = Instant::now();

        for query in &eval {
            dijkstra.find_path(
                query.start_id.unwrap(),
                query.end_id.unwrap(),
                query.alpha.clone(),
                &data.graph,
                &data.nodes,
            );
        }

        println!(
            "Dijkstra in: {:?} on average {:?}",
            dijkstra_time.elapsed(),
            dijkstra_time.elapsed().div_f64(eval.len() as f64)
        );
    } else {
        let mut dijkstra = Dijkstra::new(amount_nodes, dim, RealExport::new());

        for query in &eval {
            let result = dijkstra.find_path(
                query.start_id.unwrap(),
                query.end_id.unwrap(),
                query.alpha.clone(),
                &data.graph,
                &data.nodes,
            );
            let path = result.unwrap_or((vec![], 0.0));

            // collect
            match export_wkt::write_file(
                &format!("{}/{}.wkt", export_path, query.id),
                query.start_id.unwrap(),
                query.end_id.unwrap(),
                dijkstra.exporter.meeting_node,
                &dijkstra.exporter.visited_nodes,
                &path.0,
                &dijkstra.exporter.visited_edges,
                &data.nodes,
                &data.graph.edges,
            ) {
                Ok(_result) => (),
                Err(error) => panic!("error exporting wkt-file: {:?}", error),
            }
        }

        println!("exported successfully at {}", export_path);
    }
}

fn get_arguments() -> (String, String, bool, String) {
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
        .arg(
            clap::Arg::with_name("export")
                .help("the query should get exported")
                .takes_value(true)
                .short("x")
                .long("export"),
        )
        .get_matches();

    let mut export_path = "";
    if matches.is_present("export") {
        export_path = matches.value_of("export").unwrap();
    }

    (
        matches.value_of("fmi-file").unwrap().to_string(),
        matches.value_of("eval-file").unwrap().to_string(),
        matches.is_present("export"),
        export_path.to_string(),
    )
}

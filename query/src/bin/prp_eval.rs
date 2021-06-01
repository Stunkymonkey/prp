// use rayon::prelude::*;
use serde::Serialize;
use serde_json::json;
use std::fs::File;
use std::str::FromStr;
use std::time::{Duration, Instant};

// changing the import changes the dijkstra query method
use prp_query::dijkstra::prp::Dijkstra;
// use prp_query::dijkstra::crp::Dijkstra;
// use prp_query::dijkstra::pch::Dijkstra;
// use prp_query::dijkstra::bidirectional::Dijkstra;
// use prp_query::dijkstra::normal::Dijkstra;
use prp_query::query_export::*;
use prp_query::*;

enum Vals {
    Time,
    Count,
    Export,
    Check,
}

impl FromStr for Vals {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "time" => Ok(Vals::Time),
            "count" => Ok(Vals::Count),
            "export" => Ok(Vals::Export),
            "check" => Ok(Vals::Check),
            _ => Err("no match"),
        }
    }
}

#[derive(Debug, Serialize)]
struct TimeExport {
    id: usize,
    time: Duration,
}
#[derive(Debug, Serialize)]
struct CounterExport {
    id: usize,
    heap_pops: usize,
    relaxed_edges: usize,
}

fn main() {
    let (fmi_file, eval_file, eval_type, export_path) = get_arguments();
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
        mlp_layers: data.mlp_layers,
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

    match eval_type {
        Vals::Time => {
            let mut dijkstra = Dijkstra::new(amount_nodes, NoOp::new());
            let mut export_list: Vec<TimeExport> = Vec::with_capacity(eval.len());

            for query in &eval {
                let dijkstra_time = Instant::now();
                dijkstra.find_path(
                    query.start_id.unwrap(),
                    query.end_id.unwrap(),
                    query.alpha.clone(),
                    &data.graph,
                    &data.nodes,
                    &data.mlp_layers,
                );
                export_list.push(TimeExport {
                    id: query.id,
                    time: dijkstra_time.elapsed(),
                });
            }

            //export
            let output = serde_json::to_string_pretty(&json!({
                "total_time": export_list.iter().map(|e| e.time).sum::<Duration>(),
                "average_time": export_list.iter().map(|e| e.time).sum::<Duration>().div_f64(eval.len() as f64),
                "querys": export_list
            }))
            .unwrap();

            match export_path {
                Some(path) => match export::write_file(&path, &output) {
                    Ok(_) => println!("exported succesfully"),
                    Err(err) => println!("error while exporting {:?}", err),
                },
                None => println!("{}", output),
            }
        }
        Vals::Count => {
            let mut dijkstra = Dijkstra::new(amount_nodes, Counter::new());
            let mut export_list: Vec<CounterExport> = Vec::with_capacity(eval.len());

            for query in &eval {
                let _result = dijkstra.find_path(
                    query.start_id.unwrap(),
                    query.end_id.unwrap(),
                    query.alpha.clone(),
                    &data.graph,
                    &data.nodes,
                    &data.mlp_layers,
                );
                export_list.push(CounterExport {
                    id: query.id,
                    heap_pops: dijkstra.exporter.heap_pops,
                    relaxed_edges: dijkstra.exporter.relaxed_edges,
                });
            }

            //export
            let output = serde_json::to_string_pretty(&json!({
                "heap_pop_sum": export_list.iter().map(|e| e.heap_pops).sum::<usize>(),
                "relaxed_edge_sum": export_list.iter().map(|e| e.relaxed_edges).sum::<usize>(),
                "querys": export_list,
            }))
            .unwrap();

            match export_path {
                Some(path) => match export::write_file(&path, &output) {
                    Ok(_) => println!("exported succesfully"),
                    Err(err) => println!("error while exporting {:?}", err),
                },
                None => println!("{}", output),
            }
        }
        Vals::Export => {
            let mut dijkstra = Dijkstra::new(amount_nodes, RealExport::new());

            for query in &eval {
                let result = dijkstra.find_path(
                    query.start_id.unwrap(),
                    query.end_id.unwrap(),
                    query.alpha.clone(),
                    &data.graph,
                    &data.nodes,
                    &data.mlp_layers,
                );
                let path = result.unwrap_or((vec![], 0.0));

                //export
                match export_path {
                    Some(ref export_path) => {
                        match export::write_wkt_file(
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
                            Ok(_result) => println!("exported successfully at {}", export_path),
                            Err(error) => println!("error exporting wkt-file: {:?}", error),
                        }
                    }
                    None => println!("unable to export: no export-path given"),
                }
            }
        }
        Vals::Check => {
            let mut dijkstra = dijkstra::normal::Dijkstra::new(amount_nodes, NoOp::new());
            let mut prp_dijkstra = Dijkstra::new(amount_nodes, NoOp::new());
            let mut correct = 0;
            let mut not_correct = 0;
            let mut no_path_found = 0;
            let mut not_no_path_found = 0;
            for query in &eval {
                let result = dijkstra.find_path(
                    query.start_id.unwrap(),
                    query.end_id.unwrap(),
                    query.alpha.clone(),
                    &data.graph,
                    &data.nodes,
                    &data.mlp_layers,
                );
                let prp_result = prp_dijkstra.find_path(
                    query.start_id.unwrap(),
                    query.end_id.unwrap(),
                    query.alpha.clone(),
                    &data.graph,
                    &data.nodes,
                    &data.mlp_layers,
                );
                match (result, prp_result) {
                    (Some(result), Some(prp_result)) => {
                        // only check costs of paths, because there can be multiple paths with same value
                        if (cost_of_path(&query.alpha, &result.0, &data.graph)
                            - cost_of_path(&query.alpha, &prp_result.0, &data.graph))
                        .abs()
                            < 1.0
                        {
                            correct += 1;
                        } else {
                            not_correct += 1;
                            println!(
                                "{:?} from: {:?} to {:?} \tdij: {:?}/{:?} \talt: {:?}/{:?}",
                                query.id,
                                query.start_id.unwrap(),
                                query.end_id.unwrap(),
                                result.1,
                                cost_of_path(&query.alpha, &result.0, &data.graph),
                                prp_result.1,
                                cost_of_path(&query.alpha, &prp_result.0, &data.graph),
                            );
                        }
                    }
                    (None, None) => {
                        no_path_found += 1;
                    }
                    (None, Some(_prp_result)) => {
                        not_no_path_found += 1;
                    }
                    (Some(_result), None) => not_correct += 1,
                }
            }

            //export
            let output = serde_json::to_string_pretty(&json!({
                "correct": {
                    "with path": correct,
                    "no_path": no_path_found
                },
                "incorrect": {
                    "with_path": not_correct,
                    "no_path": not_no_path_found,
                },
            }))
            .unwrap();

            match export_path {
                Some(path) => match export::write_file(&path, &output) {
                    Ok(_) => println!("exported succesfully"),
                    Err(err) => println!("error while exporting {:?}", err),
                },
                None => println!("{}", output),
            }
        }
    }
}

fn cost_of_path(alpha: &[Cost], path: &[EdgeId], graph: &Graph) -> f64 {
    let mut cost: f64 = 0.0;
    for edge in path {
        cost += mch::costs_by_alpha(&graph.get_edge_costs(*edge), &alpha);
    }
    cost
}

fn get_arguments() -> (String, String, Vals, Option<String>) {
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
            clap::Arg::with_name("type")
                .help("What kind of evaluation will be done")
                .takes_value(true)
                .short("t")
                .long("type")
                .required(true)
                .possible_values(&["time", "count", "export", "check"]),
        )
        .arg(
            clap::Arg::with_name("export-path")
                .help("where to export to")
                .takes_value(true)
                .short("x")
                .long("export"),
        )
        .get_matches();

    let eval_type = clap::value_t!(matches.value_of("type"), Vals).unwrap_or_else(|e| e.exit());

    (
        matches.value_of("fmi-file").unwrap().to_string(),
        matches.value_of("eval-file").unwrap().to_string(),
        eval_type,
        matches.value_of("export-path").map(String::from),
    )
}

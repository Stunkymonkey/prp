#[macro_use]
extern crate log;

use actix_web::{get, middleware, post, web, App, HttpServer};
use rayon::prelude::*;
use std::cell::RefCell;
use std::path::Path;
use std::time::Instant;

use prp_query::geojson::*;
use prp_query::query_export::*;
use prp_query::*;

#[post("/dijkstra")]
async fn shortest_path(
    request: web::Json<GeoJsonRequest>,
    data: web::Data<WebData>,
    dijkstra_cell: web::Data<RefCell<Box<dyn FindPath<NoOp>>>>,
) -> Result<web::Json<GeoJsonResponse>, geojson::Error> {
    let total_time = Instant::now();

    // extract points
    let features = &request.features;
    assert_eq!(features.len(), 2);

    let start_feature = &features[0].geometry.coordinates;
    let end_feature = &features[1].geometry.coordinates;
    assert_eq!(start_feature.len(), 2);
    assert_eq!(end_feature.len(), 2);

    let start = Location {
        longitude: start_feature[0],
        latitude: start_feature[1],
    };
    let end = Location {
        longitude: end_feature[0],
        latitude: end_feature[1],
    };
    // find alpha as property at any node from last node to front
    let mut alpha_option = None;
    for feature in features.iter().rev() {
        alpha_option = match &feature.properties {
            Some(properties) => properties.alpha.clone(),
            None => alpha_option,
        };
    }
    // return Error if no alpha is set
    if alpha_option.is_none() {
        return Err(geojson::Error {
            msg: "alpha not found".to_string(),
            status: 400,
        });
    }
    let alpha = alpha_option.unwrap();
    if alpha.len() != data.graph.dim {
        return Err(geojson::Error {
            msg: "alpha vector-size does not match".to_string(),
            status: 400,
        });
    }

    debug!("Start: {},{}", start.latitude, start.longitude);
    debug!("End: {},{}", end.latitude, end.longitude);
    debug!("Alpha: {:?}", alpha);

    // search for clicked points
    let grid_time = Instant::now();
    let start_id: NodeId = grid::get_closest_point(
        start,
        &data.nodes,
        &data.grid,
        &data.grid_offset,
        &data.grid_bounds,
    );
    let end_id: NodeId = grid::get_closest_point(
        end,
        &data.nodes,
        &data.grid,
        &data.grid_offset,
        &data.grid_bounds,
    );
    debug!("start_id {}", start_id);
    debug!("end_id {}", end_id);
    info!(" Get node-ID in: {:?}", grid_time.elapsed());

    let mut dijkstra = dijkstra_cell.borrow_mut();

    let dijkstra_time = Instant::now();
    let tmp = dijkstra.find_path(
        start_id,
        end_id,
        alpha,
        &data.graph,
        &data.nodes,
        &data.mlp_levels,
    );
    info!("    Dijkstra in: {:?}", dijkstra_time.elapsed());

    let (result_path, cost): (Vec<(Angle, Angle)>, String) = match tmp {
        Some((path, path_cost)) => {
            let nodes = grid::get_coordinates(
                convert_edge_ids_to_node_ids(&path, &data.graph),
                &data.nodes,
            );
            (
                nodes
                    .par_iter()
                    .map(|node| (node.longitude, node.latitude))
                    .collect::<Vec<(Angle, Angle)>>(),
                format!("{:.2}", path_cost),
            )
        }
        None => {
            warn!("no path found");
            (Vec::<(Angle, Angle)>::new(), "no path found".to_string())
        }
    };

    info!("        Overall: {:?}", total_time.elapsed());

    Ok(web::Json(GeoJsonResponse {
        // escaping the rust-type command to normal type string
        r#type: "FeatureCollection".to_string(),
        features: vec![FeatureResponse {
            r#type: "Feature".to_string(),
            geometry: GeometryResponse {
                r#type: "LineString".to_string(),
                coordinates: result_path,
            },
            properties: Some(Property {
                cost: Some(cost),
                alpha: None,
            }),
        }],
    }))
}

#[get("/metrics")]
async fn metrics(
    data: web::Data<WebData>,
    _dijkstra_cell: web::Data<RefCell<Box<dyn FindPath<NoOp>>>>,
) -> web::Json<Vec<String>> {
    web::Json(data.metrics.clone())
}

fn convert_edge_ids_to_node_ids(edges: &[EdgeId], graph: &Graph) -> Vec<NodeId> {
    if edges.is_empty() {
        return vec![];
    }
    let mut path: Vec<NodeId> = edges
        .iter()
        .map(|edge_id| graph.edges[*edge_id].from)
        .collect();
    path.push(graph.edges[*edges.last().unwrap()].to);
    path
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let (fmi_file, port, query_type) = get_arguments();
    // read binfile
    let mut data: BinFile = match bin_import::read_file(&fmi_file) {
        Ok(result) => result,
        Err(error) => panic!("error while reading bin-file: {:?}", error),
    };

    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let amount_nodes = data.nodes.len();
    let dim = data.edge_costs.len() / data.edges.len();

    sort_edges::sort_edges(query_type, &mut data);

    let graph = Graph::new(
        data.edges,
        data.edge_costs,
        data.up_offset,
        data.down_offset,
        data.down_index,
        dim,
    );
    let data_ref = web::Data::new(WebData {
        nodes: data.nodes,
        mlp_levels: data.mlp_levels,
        graph,
        grid_offset: data.grid_offset,
        grid: data.grid,
        grid_bounds: data.grid_bounds,
        metrics: data.metrics,
    });

    // check for static-html folder
    let html_path;
    if Path::new("./html").exists() {
        html_path = "./html";
    } else if Path::new("./query/html").exists() {
        html_path = "./query/html";
    } else {
        eprintln!("<html> directory not found");
        std::process::exit(1);
    }

    // start webserver
    println!("Starting server at: http://localhost:{}", port);
    HttpServer::new(move || {
        // initialize thread-local dijkstra
        let dijkstra = RefCell::new(prp_query::dijkstra::get(
            query_type,
            amount_nodes,
            NoOp::new(),
        ));
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::JsonConfig::default().limit(1024))
            .app_data(data_ref.clone())
            .app_data(dijkstra)
            .service(shortest_path)
            .service(metrics)
            .service(actix_files::Files::new("/", html_path).index_file("index.html"))
    })
    .bind(format!("localhost:{}", port))
    .expect("Can not bind to port")
    .run()
    .await
}

fn get_arguments() -> (String, String, QueryType) {
    let matches = clap::Command::new("prp_web")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about("provides webinterface and testing option")
        .arg(
            clap::Arg::new("fmi-file")
                .help("the input file to use")
                .takes_value(true)
                .short('f')
                .long("file")
                .required(true),
        )
        .arg(
            clap::Arg::new("port")
                .help("the port the webserver will bind to")
                .takes_value(true)
                .short('p')
                .long("port")
                .default_value("8080"),
        )
        .arg(
            clap::Arg::new("query")
                .help("What type of query will be used")
                .takes_value(true)
                .short('q')
                .long("query")
                .required(true)
                .possible_values(&["normal", "bi", "pch", "pcrp", "prp"]),
        )
        .get_matches();
    let query_type = matches
        .value_of_t::<QueryType>("query")
        .unwrap_or_else(|e| e.exit());

    (
        matches.value_of("fmi-file").unwrap().to_string(),
        matches.value_of("port").unwrap().to_string(),
        query_type,
    )
}

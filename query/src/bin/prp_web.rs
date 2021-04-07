#[macro_use]
extern crate log;

use actix_web::{get, middleware, post, web, App, HttpServer};
use rayon::prelude::*;
use std::cell::RefCell;
use std::path::Path;
use std::time::Instant;

use prp_query::geojson::*;
use prp_query::*;

#[post("/dijkstra")]
async fn shortest_path(
    request: web::Json<GeoJsonRequest>,
    data: web::Data<BinFile>,
    dijkstra_cell: web::Data<RefCell<Dijkstra>>,
) -> web::Json<GeoJsonRespone> {
    let total_time = Instant::now();

    // extract points
    let features = &request.features;
    assert_eq!(features.len(), 2);

    let start_feature = &features[0].geometry.coordinates;
    let end_feature = &features[1].geometry.coordinates;
    assert_eq!(start_feature.len(), 2);
    assert_eq!(end_feature.len(), 2);

    let start = Node {
        longitude: start_feature[0],
        latitude: start_feature[1],
        rank: INVALID_RANK,
        layer_height: INVALID_LAYER_HEIGHT,
    };
    let end = Node {
        longitude: end_feature[0],
        latitude: end_feature[1],
        rank: INVALID_RANK,
        layer_height: INVALID_LAYER_HEIGHT,
    };
    debug!("Start: {},{}", start.latitude, start.longitude);
    debug!("End: {},{}", end.latitude, end.longitude);

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
        &data.nodes,
        &data.edges,
        &data.up_offset,
        &data.down_offset,
        &data.down_index,
    );
    info!("    Dijkstra in: {:?}", dijkstra_time.elapsed());

    let result: Vec<(f32, f32)>;
    let mut cost: String = "".to_string();
    match tmp {
        Some((path, path_cost)) => {
            let nodes = grid::get_coordinates(path, &data.nodes);
            result = nodes
                .par_iter()
                .map(|node| (node.longitude, node.latitude))
                .collect::<Vec<(f32, f32)>>();
            // match data.optimized_by {
            //     OptimizeBy::Time => {
            //         if path_cost.trunc() >= 1.0 {
            //             cost = path_cost.trunc().to_string();
            //             cost.push_str(" h ");
            //         }
            //         cost.push_str(&format!("{:.0}", path_cost.fract() * 60.0));
            //         cost.push_str(" min");
            //     }
            //     OptimizeBy::Distance => {
            //         cost = format!("{:.2}", path_cost);
            //         cost.push_str(" km");
            //     }
            // };
        }
        None => {
            warn!("no path found");
            result = Vec::<(f32, f32)>::new();
            cost = "no path found".to_string();
        }
    }

    info!("        Overall: {:?}", total_time.elapsed());

    return web::Json(GeoJsonRespone {
        // escaping the rust-type command to normal type string
        r#type: "FeatureCollection".to_string(),
        features: vec![FeatureResponse {
            r#type: "Feature".to_string(),
            geometry: GeometryResponse {
                r#type: "LineString".to_string(),
                coordinates: result,
            },
            properties: Some(Property { weight: cost }),
        }],
    });
}

#[get("/metrics")]
async fn metrics(
    data: web::Data<BinFile>,
    _dijkstra_cell: web::Data<RefCell<Dijkstra>>,
) -> web::Json<Vec<String>> {
    return web::Json(data.metrics.clone());
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let (fmi_file, port) = get_arguments();
    // read binfile
    let data: BinFile = match bin_import::read_file(&fmi_file) {
        Ok(result) => result,
        Err(error) => panic!("error while reading bin-file: {:?}", error),
    };

    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let amount_nodes = data.nodes.len();
    let data_ref = web::Data::new(data);

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
        let dijkstra = RefCell::new(Dijkstra::new(amount_nodes));
        App::new()
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(1024))
            .app_data(data_ref.clone())
            .data(dijkstra)
            .service(shortest_path)
            .service(metrics)
            .service(actix_files::Files::new("/", html_path).index_file("index.html"))
    })
    .bind(format!("localhost:{}", port))?
    .run()
    .await
}

fn get_arguments() -> (String, String) {
    let matches = clap::App::new("prp_web")
        .version("0.1.0")
        .about("provides webinterface and testing option")
        .author("Felix Bühler")
        .arg(
            clap::Arg::with_name("fmi-file")
                .help("the input file to use")
                .takes_value(true)
                .short("f")
                .long("file")
                .required(true),
        )
        .arg(
            clap::Arg::with_name("port")
                .help("the port the webserver will bind to")
                .takes_value(true)
                .short("p")
                .long("port")
                .default_value("8080"),
        )
        .get_matches();

    (
        matches.value_of("fmi-file").unwrap().to_string(),
        matches.value_of("port").unwrap().to_string(),
    )
}

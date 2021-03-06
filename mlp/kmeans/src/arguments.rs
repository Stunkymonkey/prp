use clap::{values_t, App, Arg};

pub fn get_arguments() -> clap::Result<(String, String, Vec<usize>)> {
    let matches = App::new("mlp_kmeans")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about("generates multi-layer-partition using kmeans")
        .arg(
            Arg::with_name("graph-file")
                .help("the input file to use")
                .takes_value(true)
                .short("f")
                .long("file")
                .required(true),
        )
        .arg(
            Arg::with_name("mlp-file")
                .help("the output file")
                .takes_value(true)
                .short("o")
                .long("output")
                .required(true),
        )
        .arg(
            Arg::with_name("partitions")
                .help("how much (size of parameters) and often (amount of parameters) the graph is divided (from top to bottom)")
                .takes_value(true)
                .multiple(true)
                .short("p")
                .long("partitions")
                .required(true),
        )
        .get_matches();

    let mut partitions = vec![];
    for val in values_t!(matches, "partitions", usize).unwrap_or_else(|e| e.exit()) {
        // println!("divide: {}", val);
        partitions.push(val);
    }

    let fmi_file = matches.value_of("graph-file").unwrap();
    let mlp_file = matches.value_of("mlp-file").unwrap();

    Ok((fmi_file.to_string(), mlp_file.to_string(), partitions))
}

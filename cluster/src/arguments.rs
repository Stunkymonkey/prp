use clap::{App, Arg};

pub fn get_arguments() -> clap::Result<(String, String, Vec<usize>)> {
    let matches = App::new("prp-cluster")
        .version("1.0.0")
        .about("generates multi-layer-partition")
        .author("Felix Bühler")
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
            Arg::with_name("clusters")
                .help("how much (size of parameters) and often (amount of parameters) the graph is divided (from bottom to up)")
                .takes_value(true)
                .multiple(true)
                .required(true),
        )
        .get_matches();

    let mut clusters = vec![];
    for val in values_t!(matches, "clusters", usize).unwrap_or_else(|e| e.exit()) {
        // println!("divide: {}", val);
        clusters.push(val);
    }

    let fmi_file = matches.value_of("graph-file").unwrap();
    let mlp_file = matches.value_of("mlp-file").unwrap();

    Ok((fmi_file.to_string(), mlp_file.to_string(), clusters))
}

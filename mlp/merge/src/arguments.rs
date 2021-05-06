use clap::{values_t, App, Arg};

pub fn get_arguments() -> clap::Result<(String, String, Vec<usize>, Vec<usize>)> {
    let matches = App::new("prp-merge")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about("generates multi-layer-partition")
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
                .help("how much (size of parameters) and often (amount of parameters) the graph is divided (from bottom to up)")
                .takes_value(true)
                .multiple(true)
                .short("p")
                .long("partitions")
                .conflicts_with("sizes")
                .required_unless("sizes"),
        )
        .arg(
            Arg::with_name("sizes")
                .help("how much nodes should be in each partition (from bottom to up)")
                .takes_value(true)
                .multiple(true)
                .short("s")
                .long("sizes")
                .conflicts_with("partitions")
                .required_unless("partitions"),
        )
        .get_matches();

    let mut partitions = vec![];
    if values_t!(matches, "partitions", usize).is_ok() {
        for val in values_t!(matches, "partitions", usize).unwrap() {
            // println!("divide: {}", val);
            partitions.push(val);
        }
    }

    let mut sizes = vec![];
    if values_t!(matches, "sizes", usize).is_ok() {
        for val in values_t!(matches, "sizes", usize).unwrap() {
            // println!("divide: {}", val);
            sizes.push(val);
        }
    }

    let fmi_file = matches.value_of("graph-file").unwrap();
    let mlp_file = matches.value_of("mlp-file").unwrap();

    Ok((
        fmi_file.to_string(),
        mlp_file.to_string(),
        partitions,
        sizes,
    ))
}

use clap::{Arg, Command, value_parser};

pub fn get_arguments() -> clap::error::Result<(String, String, Vec<usize>, Vec<usize>)> {
    let matches = Command::new("mlp_merge")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about("generates multi-level-partition using merge-algo")
        .arg(
            Arg::new("graph-file")
                .help("the input file to use")
                .num_args(1)
                .short('f')
                .long("file")
                .required(true),
        )
        .arg(
            Arg::new("mlp-file")
                .help("the output file")
                .num_args(1)
                .short('o')
                .long("output")
                .required(true),
        )
        .arg(
            Arg::new("partitions")
                .help("how many partitions (size of parameters) and often (amount of parameters) the graph is stored (from top to bottom)")
                .num_args(1..)
                .short('p')
                .long("partitions")
                .conflicts_with("sizes")
                .required_unless_present("sizes")
                .value_parser(value_parser!(usize)),
        )
        .arg(
            Arg::new("sizes")
                .help("how much nodes should be in each partition (from bottom to up)")
                .num_args(1..)
                .short('s')
                .long("sizes")
                .conflicts_with("partitions")
                .required_unless_present("partitions")
                .value_parser(value_parser!(usize)),
        )
        .get_matches();

    let partitions = if matches.get_many::<usize>("partitions").is_some() {matches.get_many("partitions").unwrap().copied().collect()} else {vec![]};
    let sizes = if matches.get_many::<usize>("sizes").is_some() {matches.get_many("sizes").unwrap().copied().collect()} else {vec![]};

    let fmi_file = matches.get_one::<String>("graph-file").expect("`graph-file` is required");
    let mlp_file = matches.get_one::<String>("mlp-file").expect("`mlp-file` is required");

    Ok((
        fmi_file.to_string(),
        mlp_file.to_string(),
        partitions,
        sizes,
    ))
}

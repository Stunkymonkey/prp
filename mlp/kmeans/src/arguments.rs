use clap::{Arg, Command, value_parser};

pub fn get_arguments() -> clap::error::Result<(String, String, Vec<usize>)> {
    let matches = Command::new("mlp_kmeans")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about("generates multi-level-partition using kmeans")
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
                .help("how much (size of parameters) and often (amount of parameters) the graph is divided (from top to bottom)")
                .num_args(1..)
                .short('p')
                .long("partitions")
                .required(true)
                .value_parser(value_parser!(usize)),
        )
        .get_matches();

    let partitions = matches
        .get_many("partitions")
        .expect("`partitions` are required").copied().collect();

    let fmi_file = matches.get_one::<String>("graph-file").expect("`graph-file` is required");
    let mlp_file = matches.get_one::<String>("mlp-file").expect("`mlp-file` is required");

    Ok((fmi_file.to_string(), mlp_file.to_string(), partitions))
}

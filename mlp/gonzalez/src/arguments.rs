use clap::{Arg, Command};

pub fn get_arguments() -> clap::Result<(String, String, Vec<usize>)> {
    let matches = Command::new("mlp_hop_distance")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about("generates multi-level-partition using merge-algo")
        .arg(
            Arg::new("graph-file")
                .help("the input file to use")
                .takes_value(true)
                .short('f')
                .long("file")
                .required(true),
        )
        .arg(
            Arg::new("mlp-file")
                .help("the output file")
                .takes_value(true)
                .short('o')
                .long("output")
                .required(true),
        )
        .arg(
            Arg::new("partitions")
                .help("how much (size of parameters) and often (amount of parameters) the graph is divided (from top to bottom)")
                .takes_value(true)
                .multiple_occurrences(true)
                .short('p')
                .long("partitions")
                .required(true),
        )
        .get_matches();

    let mut partitions = vec![];
    if matches.values_of_t::<usize>("partitions").is_ok() {
        for val in matches.values_of_t("partitions").unwrap() {
            partitions.push(val);
        }
    }

    let fmi_file = matches.value_of("graph-file").unwrap();
    let mlp_file = matches.value_of("mlp-file").unwrap();

    Ok((fmi_file.to_string(), mlp_file.to_string(), partitions))
}

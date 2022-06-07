use clap::{Arg, Command};

pub fn get_arguments() -> clap::Result<(String, String, Vec<usize>, Vec<usize>)> {
    let matches = Command::new("mlp_merge")
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
                .help("how many partitions (size of parameters) and often (amount of parameters) the graph is stored (from top to bottom)")
                .takes_value(true)
                .multiple_occurrences(true)
                .short('p')
                .long("partitions")
                .conflicts_with("sizes")
                .required_unless_present("sizes"),
        )
        .arg(
            Arg::new("sizes")
                .help("how much nodes should be in each partition (from bottom to up)")
                .takes_value(true)
                .multiple_occurrences(true)
                .short('s')
                .long("sizes")
                .conflicts_with("partitions")
                .required_unless_present("partitions"),
        )
        .get_matches();

    let mut partitions = vec![];
    if matches.values_of_t::<usize>("partitions").is_ok() {
        for val in matches.values_of_t::<usize>("partitions").unwrap() {
            // println!("divide: {}", val);
            partitions.push(val);
        }
    }

    let mut sizes = vec![];
    if matches.values_of_t::<usize>("sizes").is_ok() {
        for val in matches.values_of_t::<usize>("sizes").unwrap() {
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

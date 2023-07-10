use clap::{crate_authors, crate_version, Arg, Command};

pub fn get_arguments() -> clap::error::Result<(String, String, f64, String)> {
    let matches = Command::new("prp-pre")
        .version(crate_version!())
        .author(crate_authors!())
        .about("generates overlay-graph")
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
                .help("the multi-level-partition file")
                .num_args(1)
                .short('m')
                .long("mlp")
                .conflicts_with("contraction-stop")
                .required_unless_present("contraction-stop"),
        )
        .arg(
            Arg::new("contraction-stop")
                .help("how much nodes are contracted")
                .num_args(1)
                .short('p')
                .long("contraction-stop")
                .conflicts_with("mlp-file")
                .required_unless_present("mlp-file"),
        )
        .arg(
            Arg::new("output-file")
                .help("the output file")
                .num_args(1)
                .short('o')
                .long("output")
                .required(true),
        )
        .get_matches();

    let fmi_file = matches.get_one::<String>("graph-file").expect("`graph-file` is required");
    let binding = "".to_string();
    let mlp_file = matches.get_one::<String>("mlp-file").unwrap_or(&binding);

    let contraction_stop = match matches.get_one::<String>("mlp-file") {
        Some(_value) => 1.0,
        None => *matches
            .get_one::<f64>("contraction-stop")
            .expect("unable to read `contraction-stop` parameter"),
    };
    let output_file = matches.get_one::<String>("output-file").expect("`output-file` is required");

    Ok((
        fmi_file.to_string(),
        mlp_file.to_string(),
        contraction_stop,
        output_file.to_string(),
    ))
}

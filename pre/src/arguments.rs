use clap::{crate_authors, crate_version, Arg, Command};

pub fn get_arguments() -> clap::Result<(String, String, f64, String)> {
    let matches = Command::new("prp-pre")
        .version(crate_version!())
        .author(crate_authors!())
        .about("generates overlay-graph")
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
                .help("the multi-level-partition file")
                .takes_value(true)
                .short('m')
                .long("mlp")
                .conflicts_with("contraction-stop")
                .required_unless_present("contraction-stop"),
        )
        .arg(
            Arg::new("contraction-stop")
                .help("how much nodes are contracted")
                .takes_value(true)
                .short('p')
                .long("contraction-stop")
                .conflicts_with("mlp-file")
                .required_unless_present("mlp-file"),
        )
        .arg(
            Arg::new("output-file")
                .help("the output file")
                .takes_value(true)
                .short('o')
                .long("output")
                .required(true),
        )
        .get_matches();

    let fmi_file = matches.value_of("graph-file").unwrap();
    let mlp_file = matches.value_of("mlp-file").unwrap_or("");
    let contraction_stop = match matches.value_of("mlp-file") {
        Some(_value) => 1.0,
        None => matches
            .values_of_t::<f64>("contraction-stop")
            .unwrap_or_else(|e| e.exit())[0],
    };
    let output_file = matches.value_of("output-file").unwrap();

    Ok((
        fmi_file.to_string(),
        mlp_file.to_string(),
        contraction_stop,
        output_file.to_string(),
    ))
}

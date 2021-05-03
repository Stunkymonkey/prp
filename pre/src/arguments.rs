use clap::{crate_authors, crate_version, App, Arg};

pub fn get_arguments() -> clap::Result<(String, String, String)> {
    let matches = App::new("prp-pre")
        .version(crate_version!())
        .author(crate_authors!())
        .about("generates overlay-graph")
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
                .help("the multi-layer-partition file")
                .takes_value(true)
                .short("m")
                .long("mlp")
                .required(true),
        )
        .arg(
            Arg::with_name("output-file")
                .help("the output file")
                .takes_value(true)
                .short("o")
                .long("output")
                .required(true),
        )
        .get_matches();

    let fmi_file = matches.value_of("graph-file").unwrap();
    let mlp_file = matches.value_of("mlp-file").unwrap();
    let output_file = matches.value_of("output-file").unwrap();

    Ok((
        fmi_file.to_string(),
        mlp_file.to_string(),
        output_file.to_string(),
    ))
}

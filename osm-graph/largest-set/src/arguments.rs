use clap::{App, Arg};

pub fn get_arguments() -> clap::Result<(String, String)> {
    let matches = App::new("fmi_largest_set")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about("extract largest disjoint set")
        .arg(
            Arg::with_name("input-file")
                .help("the input fmi-file")
                .takes_value(true)
                .short("i")
                .long("input")
                .required(true),
        )
        .arg(
            Arg::with_name("output-file")
                .help("the output fmi-file")
                .takes_value(true)
                .short("o")
                .long("output")
                .required(true),
        )
        .get_matches();

    let in_file = matches.value_of("input-file").unwrap();
    let out_file = matches.value_of("output-file").unwrap();

    Ok((in_file.to_string(), out_file.to_string()))
}

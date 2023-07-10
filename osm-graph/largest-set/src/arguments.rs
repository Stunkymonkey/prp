use clap::{Arg, Command};

pub fn get_arguments() -> clap::error::Result<(String, String)> {
    let matches = Command::new("fmi_largest_set")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about("extract largest disjoint set")
        .arg(
            Arg::new("input-file")
                .help("the input fmi-file")
                .num_args(1)
                .short('i')
                .long("input")
                .required(true),
        )
        .arg(
            Arg::new("output-file")
                .help("the output fmi-file")
                .num_args(1)
                .short('o')
                .long("output")
                .required(true),
        )
        .get_matches();

    let in_file = matches.get_one::<String>("input-file").expect("`input-file` is required");
    let out_file = matches.get_one::<String>("output-file").expect("`output-file` is required");

    Ok((in_file.to_string(), out_file.to_string()))
}

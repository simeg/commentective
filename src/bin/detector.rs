extern crate clap;
extern crate detector;

use clap::App;
use clap::Arg;
use clap::Values;
use detector::exists_on_filesystem;
use std::path::Path;

fn main() {
    let matches = App::new("Detector")
        .version("1.0.0")
        .author("Simon Egersand <s.egersand@gmail.com>")
        .about("TODO")
        .arg(
            Arg::with_name("FILE")
                .help("File to analyze")
                .required(true)
                .multiple(true)
                .validator_os(exists_on_filesystem)
                .index(1),
        )
        .get_matches();

    let args_input: Values = matches.values_of("FILE").unwrap();

    let path = Path::new("example-js.js");
    let _ = detector::run(path);
}

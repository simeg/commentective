extern crate clap;
extern crate commentective;

use clap::App;
use clap::Arg;
use clap::Values;
use commentective::utils::path::exists_on_filesystem;
use std::path::Path;

fn main() {
    let matches = App::new("Detector")
        .version("1.0.0")
        .author("Simon Egersand <s.egersand@gmail.com>")
        .about("CLI tool to find comments and commented out code")
        .arg(
            Arg::with_name("FILES")
                .help("Files to analyze")
                .required(true)
                .multiple(true)
                .validator_os(exists_on_filesystem)
                .index(1),
        )
        .get_matches();

    let values: Values = matches.values_of("FILES").unwrap();
    let paths: Vec<&Path> = values.map(|file| Path::new(file)).collect::<Vec<&Path>>();

    for comments in commentective::run(paths) {
        println!("{:?}", comments);
    }
}

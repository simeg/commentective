extern crate clap;
extern crate colored;
extern crate commentective;

use clap::App;
use clap::Arg;
use clap::Values;
use commentective::printer::Printer;
use commentective::utils::path::exists_on_filesystem;
use std::io;
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
        .arg(
            Arg::with_name("short")
                .short("s")
                .long("short")
                .help("Formats output with \"file.ext:line\" without colors")
        )
        .get_matches();

    let values: Values = matches.values_of("FILES").unwrap();
    let paths: Vec<&Path> = values.map(|file| Path::new(file)).collect::<Vec<&Path>>();

    let mut printer = Printer {
        writer: io::stdout(),
        short: matches.is_present("short"),
    };

    commentective::run(paths)
        .into_iter()
        .map(|result| printer.terminal(result))
        .for_each(drop);
}

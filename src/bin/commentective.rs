extern crate clap;
extern crate colored;
extern crate commentective;

use clap::App;
use clap::Arg;
use clap::Values;
use commentective::printer::Printer;
use commentective::utils::path::exists_on_filesystem;
use commentective::utils::string::str;
use commentective::OptionsCli;
use std::io;
use std::path::Path;
use std::process;

fn main() {
    let arg_short = Arg::with_name("short").short("s").long("short").help(
        "Formats output with \"file.ext:line\" without colors. Only outputs files with comments.",
    );
    let arg_extension = Arg::with_name("extension")
        .short("e")
        .long("extension")
        .help("Only analyze files with this extension")
        .takes_value(true);
    let arg_files = Arg::with_name("FILES")
        .help("Files to analyze")
        .required(true)
        .multiple(true)
        .validator_os(exists_on_filesystem)
        .index(1);

    let matches = App::new("commentective")
        .version("0.3.0")
        .author("Simon Egersand <s.egersand@gmail.com>")
        .about("CLI tool to find comments and commented out code")
        .arg(arg_files)
        .arg(arg_extension)
        .arg(arg_short)
        .get_matches();

    let values: Values = matches.values_of("FILES").unwrap();
    let paths: Vec<&Path> = values.map(|file| Path::new(file)).collect::<Vec<&Path>>();

    let extension: Option<String> = matches.value_of("extension").map(str); // Convert &str -> String

    let opts_cli = OptionsCli {
        extension,
        short: matches.is_present("short"),
    };

    let mut printer = Printer {
        writer: &mut io::stdout(),
        options: &opts_cli,
    };

    let successful = commentective::run(paths, &opts_cli)
        .into_iter()
        .map(|result| printer.terminal(result))
        .filter(|result| result.is_err())
        .count() == 0;

    if !successful {
        process::exit(1);
    }
}

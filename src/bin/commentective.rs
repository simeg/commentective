#[macro_use]
extern crate clap;
extern crate colored;

use clap::App;
use clap::Arg;
use clap::Values;
use commentective::printer::Printer;
use commentective::utils::path::exists_on_filesystem;
use commentective::utils::string::first_char;
use commentective::utils::string::str;
use commentective::OptionsCli;
use std::io;
use std::path::Path;
use std::process;

static ARG_NAME_SHORT: &str = "short";
static ARG_NAME_EXTENSION: &str = "extension";
static ARG_NAME_IGNORE_EMPTY: &str = "ignore-empty";
static OPT_NAME_FILES: &str = "FILES";

fn main() {
    let arg_short = Arg::with_name(ARG_NAME_SHORT)
        .short(first_char(ARG_NAME_SHORT))
        .long(ARG_NAME_SHORT)
        .about("Formats output with \"file.ext:line\" without colors. Only outputs files with comments.");

    let arg_extension = Arg::with_name(ARG_NAME_EXTENSION)
        .short(first_char(ARG_NAME_EXTENSION))
        .long(ARG_NAME_EXTENSION)
        .about("Only analyze files with this extension")
        .takes_value(true);

    let arg_ignore_empty = Arg::with_name(ARG_NAME_IGNORE_EMPTY)
        .short(first_char(ARG_NAME_IGNORE_EMPTY))
        .long(ARG_NAME_IGNORE_EMPTY)
        .about("Ignore printing files without comments");

    let opt_files = Arg::with_name(OPT_NAME_FILES)
        .about("Files to analyze")
        .required(true)
        .multiple(true)
        .validator_os(exists_on_filesystem)
        .index(1);

    let matches = App::new("commentective")
        .author(crate_authors!())
        .version(crate_version!())
        .about("CLI tool to find comments and commented out code")
        .arg(opt_files)
        .arg(arg_extension)
        .arg(arg_ignore_empty)
        .arg(arg_short)
        .get_matches();

    let values: Values = matches.values_of(OPT_NAME_FILES).unwrap();
    let paths: Vec<&Path> = values.map(|file| Path::new(file)).collect::<Vec<&Path>>();

    let extension: Option<String> = matches.value_of(ARG_NAME_EXTENSION).map(str); // Convert &str -> String

    let opts_cli = OptionsCli {
        extension,
        short: matches.is_present(ARG_NAME_SHORT),
        ignore_empty: matches.is_present(ARG_NAME_IGNORE_EMPTY),
    };

    let mut printer = Printer {
        writer: io::stdout(),
        options: &opts_cli,
    };

    let successful = commentective::run(paths, &opts_cli)
        .into_iter()
        .map(|result| printer.terminal(result))
        .filter(|result| result.is_err())
        .count()
        == 0;

    if !successful {
        process::exit(1);
    }
}

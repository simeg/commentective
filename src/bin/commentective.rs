#[macro_use]
extern crate clap;
extern crate colored;

use commentective::printer::Printer;
use commentective::utils::path::{exists_on_filesystem, is_file};
use commentective::utils::string::first_char;
use commentective::{Commentative, CommentativeOpts};

use std::io;

use clap::{Arg, Command};
use std::path::PathBuf;

static ARG_NAME_SHORT: &str = "short";
static ARG_NAME_CODE: &str = "code";
static ARG_NAME_IGNORE_EMPTY: &str = "ignore-empty";

static OPT_NAME_LANG: &str = "lang";

static ARG_NAME_FILES: &str = "FILES";

fn main() {
    let arg_short = Arg::new(ARG_NAME_SHORT)
        .short(first_char(ARG_NAME_SHORT))
        .long(ARG_NAME_SHORT)
        .help("Formats output with \"file.ext:line\" without colors. Only outputs files with comments.");

    let arg_code = Arg::new(ARG_NAME_CODE)
        .short(first_char(ARG_NAME_CODE))
        .long(ARG_NAME_CODE)
        .help("Prints the source code line with the comment");

    let arg_ignore_empty = Arg::new(ARG_NAME_IGNORE_EMPTY)
        .short(first_char(ARG_NAME_IGNORE_EMPTY))
        .long(ARG_NAME_IGNORE_EMPTY)
        .help("Ignores printing files without comments");

    let opt_lang = Arg::new(OPT_NAME_LANG)
        .short(first_char(OPT_NAME_LANG))
        .long(OPT_NAME_LANG)
        .help("Analyzes as this language. Pass the extension, e.g. 'js', 'py', 'sh'")
        .takes_value(true);

    let arg_files = Arg::new(ARG_NAME_FILES)
        .help("Files to analyze")
        .required(true)
        .multiple_values(true)
        .takes_value(true)
        .validator_os(exists_on_filesystem)
        .validator_os(is_file)
        .index(1);

    let matches = Command::new("commentective")
        .author(crate_authors!())
        .version(crate_version!())
        .about("CLI tool to find comments and commented out code")
        .arg(arg_files)
        .arg(arg_ignore_empty)
        .arg(arg_short)
        .arg(arg_code)
        .arg(opt_lang)
        .get_matches();

    // Files are verified to be existing on FS and of type file (not dir)
    let paths: Vec<PathBuf> = matches
        .values_of(ARG_NAME_FILES)
        .unwrap_or_default()
        .map(PathBuf::from)
        .collect();

    let opts_cli = CommentativeOpts {
        short: matches.is_present(ARG_NAME_SHORT),
        ignore_empty: matches.is_present(ARG_NAME_IGNORE_EMPTY),
        code: matches.is_present(ARG_NAME_CODE),
        language: matches.value_of(OPT_NAME_LANG).map(String::from),
    };

    let printer = Printer::new(io::stdout());

    let mut result = Commentative::new(printer)
        .run(paths, &opts_cli)
        .into_iter()
        .filter(Result::is_err);

    if result.next().is_some() {
        std::process::exit(1);
    }
}

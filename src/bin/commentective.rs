#[macro_use]
extern crate clap;
extern crate colored;

use clap::App;
use clap::Arg;
use clap::Values;
use commentective::printer::Printer;
use commentective::utils::path::exists_on_filesystem;
use commentective::utils::string::first_char;
use commentective::{Commentative, CommentativeOpts};
use std::io;
use std::path::PathBuf;

static ARG_NAME_SHORT: &str = "short";
static ARG_NAME_CODE: &str = "code";
static ARG_NAME_LANG: &str = "lang";
static ARG_NAME_EXTENSION: &str = "extension";
static ARG_NAME_IGNORE_EMPTY: &str = "ignore-empty";
static OPT_NAME_FILES: &str = "FILES";

fn main() {
    let arg_short = Arg::new(ARG_NAME_SHORT)
        .short(first_char(ARG_NAME_SHORT))
        .long(ARG_NAME_SHORT)
        .about("Formats output with \"file.ext:line\" without colors. Only outputs files with comments.");

    let arg_code = Arg::new(ARG_NAME_CODE)
        .short(first_char(ARG_NAME_CODE))
        .long(ARG_NAME_CODE)
        .about("Print the code with comments");

    let arg_lang = Arg::new(ARG_NAME_LANG)
        .short(first_char(ARG_NAME_LANG))
        .long(ARG_NAME_LANG)
        .about("Analyze as this language. Pass the extension, e.g. 'js', 'py', 'sh'")
        .takes_value(true);

    let arg_extension = Arg::new(ARG_NAME_EXTENSION)
        .short(first_char(ARG_NAME_EXTENSION))
        .long(ARG_NAME_EXTENSION)
        .about("Only analyze files with this extension")
        .takes_value(true);

    let arg_ignore_empty = Arg::new(ARG_NAME_IGNORE_EMPTY)
        .short(first_char(ARG_NAME_IGNORE_EMPTY))
        .long(ARG_NAME_IGNORE_EMPTY)
        .about("Ignore printing files without comments");

    let opt_files = Arg::new(OPT_NAME_FILES)
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
        .arg(arg_code)
        .arg(arg_lang)
        .get_matches();

    let files: Values = matches.values_of(OPT_NAME_FILES).unwrap();
    let paths: Vec<PathBuf> = files.map(PathBuf::from).collect();

    let extension: Option<String> = matches.value_of(ARG_NAME_EXTENSION).map(String::from);
    let language: Option<String> = matches.value_of(ARG_NAME_LANG).map(String::from);

    let opts_cli = CommentativeOpts {
        extension,
        short: matches.is_present(ARG_NAME_SHORT),
        ignore_empty: matches.is_present(ARG_NAME_IGNORE_EMPTY),
        code: matches.is_present(ARG_NAME_CODE),
        language,
    };

    let mut printer = Printer {
        writer: io::stdout(),
        options: &opts_cli,
    };

    let commentective = Commentative::with_paths(paths);

    let was_successful = commentective
        .run(&opts_cli)
        .into_iter()
        .map(|result| printer.terminal(result))
        .filter(|result| result.is_err())
        .count()
        == 0;

    if !was_successful {
        std::process::exit(1);
    }
}

extern crate console;
extern crate rayon;

use rayon::prelude::*;

use crate::language::bash;
use crate::language::c;
use crate::language::cpp;
use crate::language::csharp;
use crate::language::css;
use crate::language::golang;
use crate::language::html;
use crate::language::java;
use crate::language::javascript;
use crate::language::lua;
use crate::language::php;
use crate::language::python;
use crate::language::ruby;
use crate::language::rust;
use crate::language::scala;
use crate::language::FindResult;
use crate::language::Language;
use crate::utils::comments::noop_find_result;
use crate::utils::path::extension;
use std::io::Error;
use std::io::ErrorKind;
use std::path::Path;

pub mod language;
pub mod printer;
pub mod utils;

pub struct OptionsCli {
    pub extension: Option<String>,
    pub short: bool,
    pub ignore_empty: bool,
}

pub fn run(paths: Vec<&Path>, opts: &OptionsCli) -> Vec<Result<FindResult, Error>> {
    paths
        .par_iter()
        .map(|path| resolve_type_and_run(path, &opts))
        .collect::<Vec<Result<FindResult, Error>>>()
}

pub fn resolve_type_and_run(p: &Path, opts: &OptionsCli) -> Result<FindResult, Error> {
    let unsupported_err = Err(Error::new(
        ErrorKind::NotFound,
        "Unsupported file extension",
    ));

    if exclude_file(p, opts) {
        return Ok(noop_find_result());
    }

    match p.extension() {
        None => unsupported_err,
        Some(ext) => match ext.to_str() {
            None => panic!("Could not convert OsStr -> str"),
            Some(extension) => match extension {
                "c" => c::source(p).find(),
                "cpp" => cpp::source(p).find(),
                "cs" => csharp::source(p).find(),
                "css" => css::source(p).find(),
                "go" => golang::source(p).find(),
                "html" => html::source(p).find(),
                "java" => java::source(p).find(),
                "js" => javascript::source(p).find(),
                "lua" => lua::source(p).find(),
                "php" => php::source(p).find(),
                "py" => python::source(p).find(),
                "rb" => ruby::source(p).find(),
                "rs" => rust::source(p).find(),
                "scala" => scala::source(p).find(),
                "sh" => bash::source(p).find(),
                _ => unsupported_err,
            },
        },
    }
}

fn exclude_file(p: &Path, opts: &OptionsCli) -> bool {
    match &opts.extension {
        None => false,
        Some(ext_options) => match extension(p) {
            Ok(ext_file) => ext_options != &ext_file,
            Err(e) => panic!("{:?}", e),
        },
    }
}

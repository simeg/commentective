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
use std::path::{Path, PathBuf};

pub mod language;
pub mod printer;
pub mod utils;

pub struct OptionsCli {
    pub extension: Option<String>,
    pub short: bool,
    pub ignore_empty: bool,
}

pub fn run(paths: Vec<PathBuf>, opts: &OptionsCli) -> Vec<Result<FindResult, Error>> {
    paths
        .par_iter()
        .map(|path| resolve_type_and_run(path.to_path_buf(), &opts))
        .collect::<Vec<Result<FindResult, Error>>>()
}

pub fn resolve_type_and_run(path: PathBuf, opts: &OptionsCli) -> Result<FindResult, Error> {
    // TODO: Silently ignore directories
    let unsupported_err = Err(Error::new(
        ErrorKind::NotFound,
        "Unsupported file extension",
    ));

    if exclude_file(&path, opts) {
        return Ok(noop_find_result());
    }

    match path.extension() {
        None => unsupported_err,
        Some(ext) => match ext.to_str() {
            None => panic!("Could not convert OsStr -> str"),
            Some(extension) => match extension {
                "c" => c::source(path).find(),
                "cpp" => cpp::source(path).find(),
                "cs" => csharp::source(path).find(),
                "css" => css::source(path).find(),
                "go" => golang::source(path).find(),
                "html" => html::source(path).find(),
                "java" => java::source(path).find(),
                "js" => javascript::source(path).find(),
                "lua" => lua::source(path).find(),
                "php" => php::source(path).find(),
                "py" => python::source(path).find(),
                "rb" => ruby::source(path).find(),
                "rs" => rust::source(path).find(),
                "scala" => scala::source(path).find(),
                "sh" => bash::source(path).find(),
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

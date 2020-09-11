extern crate console;
extern crate rayon;

use rayon::prelude::*;

use crate::language::bash::Bash;
use crate::language::c::C;
use crate::language::cpp::Cpp;
use crate::language::csharp::CSharp;
use crate::language::css::CSS;
use crate::language::golang::Go;
use crate::language::html::HTML;
use crate::language::java::Java;
use crate::language::javascript::JavaScript;
use crate::language::lua::Lua;
use crate::language::php::PHP;
use crate::language::python::Python;
use crate::language::ruby::Ruby;
use crate::language::rust::Rust;
use crate::language::scala::Scala;
use crate::language::FindComment;
use crate::language::FindResult;
use crate::utils::comments::noop_find_result;
use crate::utils::path::extension;

use std::fs::metadata;
use std::io::Error;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

pub mod language;
pub mod printer;
pub mod utils;

pub struct OptsCli {
    pub extension: Option<String>,
    pub short: bool,
    pub ignore_empty: bool,
}

pub fn run(paths: Vec<PathBuf>, opts: &OptsCli) -> Vec<Result<FindResult, Error>> {
    paths
        .par_iter()
        .filter(|path| metadata(path).unwrap().is_file()) // File presence has been verified so we can unwrap here
        .map(|path| resolve_type_and_run(path.to_path_buf(), &opts))
        .collect::<Vec<Result<FindResult, Error>>>()
}

pub fn resolve_type_and_run(path: PathBuf, opts: &OptsCli) -> Result<FindResult, Error> {
    let unsupported_err = Err(Error::new(
        ErrorKind::NotFound,
        format!(
            "Unsupported file extension for path: {}",
            path.to_str().unwrap()
        ),
    ));

    if exclude_file(&path, opts) {
        return Ok(noop_find_result());
    }

    match path.extension() {
        None => unsupported_err,
        Some(ext) => match ext.to_str() {
            None => panic!("Could not convert OsStr -> str"),
            Some(extension) => match extension {
                "c" => C::default().find(path),
                "cpp" => Cpp::default().find(path),
                "cs" => CSharp::default().find(path),
                "css" => CSS::default().find(path),
                "go" => Go::default().find(path),
                "html" => HTML::default().find(path),
                "java" => Java::default().find(path),
                "js" => JavaScript::default().find(path),
                "lua" => Lua::default().find(path),
                "php" => PHP::default().find(path),
                "py" => Python::default().find(path),
                "rb" => Ruby::default().find(path),
                "rs" => Rust::default().find(path),
                "scala" => Scala::default().find(path),
                "sh" => Bash::default().find(path),
                _ => unsupported_err,
            },
        },
    }
}

fn exclude_file(p: &Path, opts: &OptsCli) -> bool {
    match &opts.extension {
        None => false,
        Some(ext_options) => match extension(p) {
            Ok(ext_file) => ext_options != &ext_file,
            Err(e) => panic!("{:?}", e),
        },
    }
}

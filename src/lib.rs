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
use crate::language::{FindResult, Finder};
use crate::utils::path::extension;

use std::ffi::OsStr;
use std::fs::metadata;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

pub mod language;
pub mod printer;
pub mod utils;

pub struct Commentative {
    paths: Vec<PathBuf>,
    finder: Finder,
}

pub struct CommentativeOpts {
    pub extension: Option<String>,
    pub short: bool,
    pub ignore_empty: bool,
    pub code: bool,
    pub language: Option<String>,
}

impl Commentative {
    pub fn with_paths(paths: Vec<PathBuf>) -> Self {
        Self {
            paths,
            finder: Finder {},
        }
    }

    pub fn run(&self, opts: &CommentativeOpts) -> Vec<Result<FindResult, Error>> {
        self.paths
            .par_iter()
            .filter(|path| metadata(path).unwrap().is_file()) // File presence has been verified so we can unwrap here
            .map(|path| self.resolve_type_and_run(path.to_path_buf(), &opts))
            .collect::<Vec<Result<FindResult, Error>>>()
    }

    fn resolve_type_and_run(
        &self,
        path: PathBuf,
        opts: &CommentativeOpts,
    ) -> Result<FindResult, Error> {
        let unsupported_err = Err(Error::new(
            ErrorKind::NotFound,
            format!(
                "Unsupported file extension for path: {}",
                path.to_str().unwrap()
            ),
        ));

        if self.exclude_file(&path, opts) {
            return Ok(self.finder.noop_find_result());
        }

        let extension = if let Some(provided_extension) = &opts.language {
            Some(provided_extension.as_str())
        } else {
            path.extension().map(OsStr::to_str).flatten()
        };

        match extension {
            None => unsupported_err,
            Some(extension) => match extension {
                "c" => C::with_finder(self.finder).find(path),
                "cpp" => Cpp::with_finder(self.finder).find(path),
                "cs" => CSharp::with_finder(self.finder).find(path),
                "css" => CSS::with_finder(self.finder).find(path),
                "go" => Go::with_finder(self.finder).find(path),
                "html" => HTML::with_finder(self.finder).find(path),
                "java" => Java::with_finder(self.finder).find(path),
                "js" => JavaScript::with_finder(self.finder).find(path),
                "lua" => Lua::with_finder(self.finder).find(path),
                "php" => PHP::with_finder(self.finder).find(path),
                "py" => Python::with_finder(self.finder).find(path),
                "rb" => Ruby::with_finder(self.finder).find(path),
                "rs" => Rust::with_finder(self.finder).find(path),
                "scala" => Scala::with_finder(self.finder).find(path),
                "sh" => Bash::with_finder(self.finder).find(path),
                _ => unsupported_err,
            },
        }
    }

    fn exclude_file(&self, path: &Path, opts: &CommentativeOpts) -> bool {
        match &opts.extension {
            None => false,
            Some(ext_options) => match extension(path) {
                Ok(ext_file) => ext_options != ext_file,
                Err(e) => panic!("{:?}", e),
            },
        }
    }
}

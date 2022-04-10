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

use crate::printer::Printer;

use std::ffi::OsStr;
use std::io;
use std::io::{Error, ErrorKind, Write};
use std::path::PathBuf;

pub mod language;
pub mod printer;
pub mod utils;

pub struct Commentative<W> {
    finder: Finder,
    printer: Printer<W>,
}

pub struct CommentativeOpts {
    pub short: bool,
    pub ignore_empty: bool,
    pub code: bool,
    pub language: Option<String>,
}

impl<W> Commentative<W>
where
    W: Write + std::marker::Sync + std::marker::Send,
{
    pub fn new(printer: Printer<W>) -> Self {
        Self {
            finder: Finder {},
            printer,
        }
    }

    pub fn run(&mut self, paths: Vec<PathBuf>, opts: &CommentativeOpts) -> Vec<io::Result<()>> {
        let find_results = paths
            // Cannot parallelize both finding and printing because printing mutates state,
            // so we do it in two steps
            .par_iter()
            .map(|path| self.resolve_type_and_run(path.to_path_buf(), opts))
            .collect::<Vec<io::Result<FindResult>>>();

        find_results
            .into_iter()
            .map(|find_result| self.print_results(find_result, opts))
            .collect()
    }

    fn resolve_type_and_run(
        &self,
        path: PathBuf,
        opts: &CommentativeOpts,
    ) -> io::Result<FindResult> {
        let extension_file = if let Some(extension_from_opts) = &opts.language {
            Some(extension_from_opts.as_str())
        } else {
            path.extension().and_then(OsStr::to_str)
        };

        match extension_file {
            None => Err(Error::new(
                ErrorKind::NotFound,
                format!("Could not identify extension for file: {}", path.display()),
            )),
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
                _ => Err(Error::new(
                    ErrorKind::NotFound,
                    format!("Unsupported file extension for path: {}", path.display()),
                )),
            },
        }
    }

    fn print_results(
        &mut self,
        find_result: io::Result<FindResult>,
        opts: &CommentativeOpts,
    ) -> io::Result<()> {
        match find_result {
            Ok(result) => {
                match result.comments.len() {
                    0 => {
                        if opts.short || opts.ignore_empty {
                            // Don't print if nothing found
                            Ok(())
                        } else {
                            self.printer.print_no_comments_found(result.file_name)
                        }
                    }
                    _ => {
                        let file_name = result.file_name;
                        if !opts.short {
                            self.printer.print_file_name(&file_name)?;
                        }

                        result.comments.into_iter().try_for_each(|line| {
                            self.printer
                                .print_comments(line, &file_name, opts.short, opts.code)
                        })
                    }
                }
            }
            Err(e) => {
                if !opts.short {
                    self.printer.print_error(&e)?;
                }
                Err(e)
            }
        }
    }
}

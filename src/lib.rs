extern crate console;
extern crate rayon;

use rayon::prelude::*;

use language::bash;
use language::c;
use language::cpp;
use language::csharp;
use language::css;
use language::golang;
use language::html;
use language::java;
use language::javascript;
use language::lua;
use language::php;
use language::python;
use language::ruby;
use language::rust;
use language::scala;
use language::FileType;
use language::FindResult;
use language::Language;
use std::io::Error;
use std::io::ErrorKind;
use std::path::Path;
use utils::comments::noop_find_result;
use utils::path::extension;

pub mod language;
pub mod printer;
pub mod utils;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct OptionsCli {
    pub extension: Option<String>,
    pub short: bool,
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
        Some(_ext) => match _ext.to_str() {
            None => panic!("Could not convert OsStr -> str"),
            Some(extension) => match extension {
                "c" => Ok(run_source(FileType::C, p)?),
                "cpp" => Ok(run_source(FileType::Cpp, p)?),
                "cs" => Ok(run_source(FileType::CSharp, p)?),
                "css" => Ok(run_source(FileType::CSS, p)?),
                "go" => Ok(run_source(FileType::Go, p)?),
                "html" => Ok(run_source(FileType::HTML, p)?),
                "java" => Ok(run_source(FileType::Java, p)?),
                "js" => Ok(run_source(FileType::JavaScript, p)?),
                "lua" => Ok(run_source(FileType::Lua, p)?),
                "php" => Ok(run_source(FileType::PHP, p)?),
                "py" => Ok(run_source(FileType::Python, p)?),
                "rb" => Ok(run_source(FileType::Ruby, p)?),
                "rs" => Ok(run_source(FileType::Rust, p)?),
                "sc" => Ok(run_source(FileType::Scala, p)?),
                "scala" => Ok(run_source(FileType::Scala, p)?),
                "sh" => Ok(run_source(FileType::Bash, p)?),
                _ => unsupported_err,
            },
        },
    }
}

fn run_source(file_type: FileType, p: &Path) -> Result<FindResult, Error> {
    match file_type {
        FileType::Bash => bash::source(p).find(),
        FileType::C => c::source(p).find(),
        FileType::CSS => css::source(p).find(),
        FileType::CSharp => csharp::source(p).find(),
        FileType::Cpp => cpp::source(p).find(),
        FileType::Go => golang::source(p).find(),
        FileType::HTML => html::source(p).find(),
        FileType::Java => java::source(p).find(),
        FileType::JavaScript => javascript::source(p).find(),
        FileType::Lua => lua::source(p).find(),
        FileType::PHP => php::source(p).find(),
        FileType::Python => python::source(p).find(),
        FileType::Ruby => ruby::source(p).find(),
        FileType::Rust => rust::source(p).find(),
        FileType::Scala => scala::source(p).find(),
    }
}

fn exclude_file(p: &Path, opts: &OptionsCli) -> bool {
    match &opts.extension {
        None => false,
        Some(ext_options) => match extension(p) {
            Ok(ext_file) => {
                if ext_options == &ext_file {
                    false
                } else {
                    true
                }
            }
            Err(e) => panic!("{:?}", e),
        },
    }
}

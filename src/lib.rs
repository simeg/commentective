extern crate console;

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
use language::FileTypes;
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

pub struct OptionsCli {
    pub extension: Option<String>,
}

pub fn run(paths: Vec<&Path>, opts: OptionsCli) -> Vec<Result<FindResult, Error>> {
    paths
        .into_iter()
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
                "c" => Ok(run_source(FileTypes::C, p)?),
                "cpp" => Ok(run_source(FileTypes::Cpp, p)?),
                "cs" => Ok(run_source(FileTypes::CSharp, p)?),
                "css" => Ok(run_source(FileTypes::CSS, p)?),
                "go" => Ok(run_source(FileTypes::Go, p)?),
                "html" => Ok(run_source(FileTypes::HTML, p)?),
                "java" => Ok(run_source(FileTypes::Java, p)?),
                "js" => Ok(run_source(FileTypes::JavaScript, p)?),
                "lua" => Ok(run_source(FileTypes::Lua, p)?),
                "php" => Ok(run_source(FileTypes::PHP, p)?),
                "py" => Ok(run_source(FileTypes::Python, p)?),
                "rb" => Ok(run_source(FileTypes::Ruby, p)?),
                "rs" => Ok(run_source(FileTypes::Rust, p)?),
                "sc" => Ok(run_source(FileTypes::Scala, p)?),
                "scala" => Ok(run_source(FileTypes::Scala, p)?),
                "sh" => Ok(run_source(FileTypes::Bash, p)?),
                _ => unsupported_err,
            },
        },
    }
}

fn run_source(file_type: FileTypes, p: &Path) -> Result<FindResult, Error> {
    match file_type {
        FileTypes::Bash => bash::source(p).find(),
        FileTypes::C => c::source(p).find(),
        FileTypes::CSS => css::source(p).find(),
        FileTypes::CSharp => csharp::source(p).find(),
        FileTypes::Cpp => cpp::source(p).find(),
        FileTypes::Go => golang::source(p).find(),
        FileTypes::HTML => html::source(p).find(),
        FileTypes::Java => java::source(p).find(),
        FileTypes::JavaScript => javascript::source(p).find(),
        FileTypes::Lua => lua::source(p).find(),
        FileTypes::PHP => php::source(p).find(),
        FileTypes::Python => python::source(p).find(),
        FileTypes::Ruby => ruby::source(p).find(),
        FileTypes::Rust => rust::source(p).find(),
        FileTypes::Scala => scala::source(p).find(),
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

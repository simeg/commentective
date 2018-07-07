use language::bash;
use language::csharp;
use language::java;
use language::javascript;
use language::php;
use language::python;
use language::rust;
use language::FileTypes;
use language::FindResult;
use language::Language;
use std::io::Error;
use std::io::ErrorKind;
use std::path::Path;

pub mod language;
pub mod utils;

pub fn run(paths: Vec<&Path>) -> Vec<Result<FindResult, Error>> {
    paths
        .into_iter()
        .map(|path| resolve_type(path))
        .collect::<Vec<Result<FindResult, Error>>>()
}

pub fn resolve_type(p: &Path) -> Result<FindResult, Error> {
    let unsupported_err = Err(Error::new(
        ErrorKind::NotFound,
        "Unsupported file extension",
    ));
    match p.extension() {
        None => unsupported_err,
        Some(_ext) => match _ext.to_str() {
            None => panic!("Could convert OsStr -> str"),
            Some(extension) => match extension.as_ref() {
                "cs" => Ok(run_source(FileTypes::CSharp, p)?),
                "java" => Ok(run_source(FileTypes::Java, p)?),
                "js" => Ok(run_source(FileTypes::JavaScript, p)?),
                "php" => Ok(run_source(FileTypes::PHP, p)?),
                "py" => Ok(run_source(FileTypes::Python, p)?),
                "rs" => Ok(run_source(FileTypes::Rust, p)?),
                "sh" => Ok(run_source(FileTypes::Bash, p)?),
                _ => unsupported_err,
            },
        },
    }
}

fn run_source(file_type: FileTypes, p: &Path) -> Result<FindResult, Error> {
    match file_type {
        FileTypes::Bash => bash::source(p).find(),
        FileTypes::CSharp => csharp::source(p).find(),
        FileTypes::Java => java::source(p).find(),
        FileTypes::JavaScript => javascript::source(p).find(),
        FileTypes::PHP => php::source(p).find(),
        FileTypes::Python => python::source(p).find(),
        FileTypes::Rust => rust::source(p).find(),
    }
}

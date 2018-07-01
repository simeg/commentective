use language::java;
use language::javascript;
use language::FileTypes;
use language::FindResult;
use language::Language;
use std::ffi::OsStr;
use std::ffi::OsString;
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
                "js" => Ok(run_source(FileTypes::JavaScript, p)?),
                "java" => Ok(run_source(FileTypes::Java, p)?),
                _ => unsupported_err,
            },
        },
    }
}

fn run_source(file_type: FileTypes, p: &Path) -> Result<FindResult, Error> {
    match file_type {
        FileTypes::JavaScript => javascript::source(p).find(),
        FileTypes::Java => java::source(p).find(),
    }
}

pub fn exists_on_filesystem(path: &OsStr) -> Result<(), OsString> {
    match path.to_str() {
        None => Err(OsString::from(format!(
            "Could not convert input file path -> str"
        ))),
        Some(p) => {
            if Path::new(p).exists() {
                return Ok(());
            }
            Err(OsString::from(format!(
                "Cannot verify that file exist [{}]",
                path.to_str().unwrap()
            )))
        }
    }
}

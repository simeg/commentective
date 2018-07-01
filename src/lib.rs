use language::javascript;
use language::javascript::FileType;
use language::FindResult;
use language::Language;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::fs::File;
use std::io::Error;
use std::io::ErrorKind;
use std::path::Path;
use utils::path::filename;

pub mod language;
pub mod utils;

pub fn run(paths: Vec<&Path>) -> Vec<Result<FindResult, Error>> {
    paths
        .into_iter()
        .map(|path| resolve_type(path).and_then(|language| language.find()))
        .collect::<Vec<Result<FindResult, Error>>>()
}

pub fn resolve_type(p: &Path) -> Result<FileType, Error> {
    let unsupported_err = Err(Error::new(
        ErrorKind::NotFound,
        "Unsupported file extension",
    ));
    match p.extension() {
        None => unsupported_err,
        Some(_ext) => match _ext.to_str() {
            None => panic!("Could convert OsStr -> str"),
            Some(extension) => match extension.as_ref() {
                "js" => Ok(javascript::FileType {
                    maybe_file: File::open(p),
                    file_name: filename(p)?,
                }),
                _ => unsupported_err,
            },
        },
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

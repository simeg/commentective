use language::javascript;
use language::javascript::Struct;
use language::FindResult;
use language::Language;
use std::fs::File;
use std::io::Error;
use std::io::ErrorKind;
use std::path::Path;

pub mod language;

pub fn run(path: &Path) -> Result<FindResult, Error> {
    resolve_type(path).map(|language| language.find())?
}

fn resolve_type(p: &Path) -> Result<Struct, Error> {
    let unsupported_err = Err(Error::new(
        ErrorKind::NotFound,
        "Unsupported file extension",
    ));
    match p.extension() {
        None => unsupported_err,
        Some(_ext) => match _ext.to_str() {
            None => panic!("Could convert OsStr -> str"),
            Some(extension) => match extension.as_ref() {
                "js" => Ok(javascript::Struct {
                    file: File::open(p),
                }),
                _ => unsupported_err,
            },
        },
    }
}

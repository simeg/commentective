use std::io::Error;
use std::fs::File;

pub mod javascript;

pub enum _SupportedLanguages {
    JavaScript,
}

pub struct FileType {
    pub maybe_file: Result<File, Error>,
    pub file_name: String,
}

pub trait Language {
    fn find(&self) -> Result<FindResult, Error>;
}

#[derive(Debug)]
pub struct FindResult {
    pub file_name: String,
    pub lines: Vec<u32>,
}

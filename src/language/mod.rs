use std::io::Error;

pub mod javascript;

pub enum _SupportedLanguages {
    JavaScript,
}

pub trait Language {
    fn find(&self) -> Result<FindResult, Error>;
}

#[derive(Debug)]
pub struct FindResult {
    pub file_name: String,
    pub lines: Vec<u32>,
}

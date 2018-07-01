use std::fs::File;
use std::io::BufReader;
use std::io::Error;

pub mod javascript;

pub enum SupportedLanguages {
    JavaScript,
}

pub trait Language {
    fn find(file: File) -> Result<Comments, Error>;
}

pub struct Comments {
    pub lines: Vec<u32>,
}

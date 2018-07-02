use std::io::Error;

pub mod java;
pub mod javascript;
pub mod rust;

pub enum FileTypes {
    JavaScript,
    Java,
    Rust,
}

pub trait Language {
    fn find(&self) -> Result<FindResult, Error>
    where
        Self: Sized;
}

#[derive(Debug)]
pub struct FindResult {
    pub file_name: String,
    pub lines: Vec<u32>,
}

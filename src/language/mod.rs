use std::io::Error;

pub mod java;
pub mod javascript;

pub enum FileTypes {
    JavaScript,
    Java,
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

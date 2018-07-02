use std::io::Error;

pub mod java;
pub mod javascript;
pub mod python;
pub mod rust;

pub enum FileTypes {
    JavaScript,
    Java,
    Rust,
    Python,
}

pub trait Language {
    fn find(&self) -> Result<FindResult, Error>
    where
        Self: Sized;
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct FindResult {
    pub file_name: String,
    pub lines: Vec<u32>,
}

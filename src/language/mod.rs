use std::io::Error;

pub mod csharp;
pub mod java;
pub mod javascript;
pub mod python;
pub mod rust;

pub enum FileTypes {
    CSharp,
    Java,
    JavaScript,
    Python,
    Rust,
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

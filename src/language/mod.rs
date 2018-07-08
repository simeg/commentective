use std::io::Error;

pub mod bash;
pub mod csharp;
pub mod java;
pub mod javascript;
pub mod php;
pub mod python;
pub mod ruby;
pub mod rust;

pub enum FileTypes {
    Bash,
    CSharp,
    Java,
    JavaScript,
    PHP,
    Python,
    Ruby,
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

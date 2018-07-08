use std::io::Error;

pub mod bash;
pub mod c;
pub mod csharp;
pub mod css;
pub mod golang;
pub mod html;
pub mod java;
pub mod javascript;
pub mod php;
pub mod python;
pub mod ruby;
pub mod rust;
pub mod scala;

pub enum FileTypes {
    Bash,
    C,
    CSS,
    CSharp,
    Go,
    HTML,
    Java,
    JavaScript,
    PHP,
    Python,
    Ruby,
    Rust,
    Scala,
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

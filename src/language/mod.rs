use std::io::Error;
use utils::string::s;

pub mod bash;
pub mod c;
pub mod cpp;
pub mod csharp;
pub mod css;
pub mod golang;
pub mod html;
pub mod java;
pub mod javascript;
pub mod lua;
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
    Cpp,
    Go,
    HTML,
    Java,
    JavaScript,
    Lua,
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
    pub print: bool,
}

impl Default for FindResult {
    fn default() -> FindResult {
        FindResult {
            file_name: s("DEFAULT_FILE_NAME"),
            lines: [].to_vec(),
            print: true,
        }
    }
}

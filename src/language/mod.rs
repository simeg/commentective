use crate::utils::string::str;
use std::io::Error;

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

pub trait Language {
    fn find(&self) -> Result<FindResult, Error>
    where
        Self: Sized;
}

pub struct FindResult {
    pub file_name: String,
    pub lines: Vec<u32>,
    pub print: bool,
}

impl Default for FindResult {
    fn default() -> FindResult {
        FindResult {
            file_name: str("DEFAULT_FILE_NAME"),
            lines: [].to_vec(),
            print: true,
        }
    }
}

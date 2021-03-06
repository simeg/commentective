use crate::language::{FindResult, Finder};
use crate::utils::path::file_name;
use crate::utils::string::contains_any_of;

use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::PathBuf;
use std::vec::Vec;

pub struct Python {
    _finder: Finder,
}

impl Python {
    pub fn with_finder(finder: Finder) -> Self {
        Self { _finder: finder }
    }

    pub fn find(&self, path: PathBuf) -> Result<FindResult, Error> {
        let mut counter = 1; // Lines begin on index 1
        let mut comments = Vec::<(u32, String)>::new();

        let file = File::open(&path)?;
        let file_name = file_name(&path)?;

        for line in BufReader::new(file).lines() {
            match line {
                Ok(l) => {
                    if self.is_comment(&l) {
                        comments.push((counter, l.to_string()));
                    }
                }
                Err(_) => panic!("Could not read line"),
            }
            counter += 1;
        }

        Ok(FindResult {
            file_name: file_name.to_string(),
            lines: comments,
            ..Default::default()
        })
    }

    fn is_comment(&self, line: &str) -> bool {
        contains_any_of(line, vec!["#"])
    }
}

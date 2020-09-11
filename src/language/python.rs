use crate::language::FindResult;
use crate::language::Language;
use crate::utils::path::file_name;
use crate::utils::string::contains_any_of;

use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::PathBuf;
use std::vec::Vec;

pub struct Python {
    pub path: PathBuf,
}

pub fn source(path: PathBuf) -> Python {
    Python { path }
}

impl Language for Python {
    fn find(&self) -> Result<FindResult, Error> {
        let mut counter = 1; // Lines begin on index 1
        let mut comments = Vec::<u32>::new();

        let file = File::open(&self.path)?;
        let file_name = file_name(&self.path)?;

        for line in BufReader::new(file).lines() {
            match line {
                Ok(l) => {
                    if is_comment(l) {
                        comments.push(counter);
                    }
                }
                Err(_) => panic!("Could not read line"),
            }
            counter += 1;
        }

        Ok(FindResult {
            file_name,
            lines: comments,
            ..Default::default()
        })
    }
}

fn is_comment(line: String) -> bool {
    contains_any_of(&line, vec!["#"])
}

use crate::language::FindResult;
use crate::language::Language;
use crate::utils::path::filename;
use crate::utils::string::string_contains_any_of;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::io::ErrorKind;
use std::path::Path;
use std::vec::Vec;

pub struct Python {
    pub maybe_file: Result<File, Error>,
    pub file_name: String,
}

pub fn source(p: &Path) -> Python {
    Python {
        maybe_file: File::open(p),
        file_name: filename(p).unwrap(),
    }
}

impl Language for Python {
    fn find(&self) -> Result<FindResult, Error> {
        let mut counter = 1; // Lines begin on index 1
        let mut comments = Vec::<u32>::new();

        match self.maybe_file {
            Ok(ref file) => {
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
                    file_name: self.file_name.to_owned(),
                    lines: comments,
                    ..Default::default()
                })
            }
            Err(_) => Err(Error::new(ErrorKind::InvalidInput, "Could not parse file")),
        }
    }
}

fn is_comment(line: String) -> bool {
    string_contains_any_of(line, vec!["#"])
}

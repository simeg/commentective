use language;
use language::FindResult;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::io::ErrorKind;
use std::vec::Vec;

pub struct Struct {
    pub file: Result<File, Error>,
}

impl language::Language for Struct {
    #[inline]
    fn find(&self) -> Result<language::FindResult, Error> {
        let mut counter = 1; // Lines begin on index 1
        let mut comments = Vec::<u32>::new();

        match self.file {
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
                    counter = counter + 1;
                }

                Ok(FindResult { lines: comments })
            }
            Err(_) => Err(Error::new(ErrorKind::InvalidInput, "Could not parse file")),
        }
    }
}

fn is_comment(line: String) -> bool {
    let first_two_chars = &line.trim().get(0..2);
    return match first_two_chars {
        &None => false,
        &Some(chars) => {
            return match chars {
                "//" => true,
                "/*" => true,
                _ => {
                    return match &chars.get(0..1) {
                        None => false,
                        Some(first_char) => {
                            return match first_char {
                                &"*" => true,
                                _ => false,
                            };
                        }
                    };
                }
            };
        }
    };
}

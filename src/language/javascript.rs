use language;
use language::Comments;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::vec::Vec;

pub struct JavaScript();

//#[inline]
//pub fn source(file: File) -> Result<JavaScript, ()> {}

impl language::Language for JavaScript {
    #[inline]
    fn find(file: File) -> Result<language::Comments, Error> {
        let mut counter = 1; // Lines begin on index 1
        let mut comments = Vec::new();

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

        Ok(Comments { lines: comments })
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

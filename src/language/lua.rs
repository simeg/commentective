use crate::language::FindResult;
use crate::language::Language;
use crate::utils::path::filename;
use crate::utils::string::str;
use crate::utils::string::string_contains_all;
use crate::utils::string::string_contains_any_of;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::io::ErrorKind;
use std::path::Path;
use std::vec::Vec;

pub struct Lua {
    pub maybe_file: Result<File, Error>,
    pub file_name: String,
}

#[derive(PartialEq, Eq)]
enum LuaCommentType {
    SingleLine,
    MultiLineStart,
    MultiLineEnd,
    None,
}

pub fn source(p: &Path) -> Lua {
    Lua {
        maybe_file: File::open(p),
        file_name: filename(p).unwrap(),
    }
}

impl Language for Lua {
    fn find(&self) -> Result<FindResult, Error> {
        let mut counter = 1; // Lines begin on index 1
        let mut comments = Vec::<u32>::new();
        let mut in_multiline = false;

        match self.maybe_file {
            Ok(ref file) => {
                for line in BufReader::new(file).lines() {
                    match line {
                        Ok(l) => {
                            let comment_type = get_comment_type(&l);
                            if in_multiline {
                                // Ignore everything except MultiLineEnd when in a multiline-comment
                                comments.push(counter);
                                in_multiline = comment_type != LuaCommentType::MultiLineEnd;
                            } else {
                                match get_comment_type(&l) {
                                    LuaCommentType::SingleLine => comments.push(counter),
                                    LuaCommentType::MultiLineStart => {
                                        in_multiline = true;
                                        comments.push(counter);
                                    }
                                    _ => (),
                                }
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

fn get_comment_type(line: &str) -> LuaCommentType {
    if string_contains_all(str(line), vec!["--[[", "]]"]) {
        return LuaCommentType::SingleLine;
    }

    if string_contains_any_of(str(line), vec!["]]"]) {
        return LuaCommentType::MultiLineEnd;
    }

    if string_contains_any_of(str(line), vec!["--[["]) {
        return LuaCommentType::MultiLineStart;
    }

    if string_contains_any_of(str(line), vec!["--"]) {
        return LuaCommentType::SingleLine;
    }

    LuaCommentType::None
}

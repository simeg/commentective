use crate::language::{FindResult, Finder};
use crate::utils::path::file_name;
use crate::utils::string::contains_all;
use crate::utils::string::contains_any_of;

use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::PathBuf;
use std::vec::Vec;

pub struct Lua {
    _finder: Finder,
}

#[derive(PartialEq, Eq)]
enum LuaCommentType {
    SingleLine,
    MultiLineStart,
    MultiLineEnd,
    None,
}

impl Lua {
    pub fn with_finder(finder: Finder) -> Self {
        Self { _finder: finder }
    }

    pub fn find(&self, path: PathBuf) -> Result<FindResult, Error> {
        let mut counter = 1; // Lines begin on index 1
        let mut comments = Vec::<(u32, String)>::new();
        let mut in_multiline = false;

        let file = File::open(&path)?;
        let file_name = file_name(&path)?;

        for line in BufReader::new(file).lines() {
            match line {
                Ok(l) => {
                    let comment_type = self.get_comment_type(&l);
                    if in_multiline {
                        // Ignore everything except MultiLineEnd when in a multiline-comment
                        comments.push((counter, l.to_string()));
                        in_multiline = comment_type != LuaCommentType::MultiLineEnd;
                    } else {
                        match self.get_comment_type(&l) {
                            LuaCommentType::SingleLine => comments.push((counter, l.to_string())),
                            LuaCommentType::MultiLineStart => {
                                in_multiline = true;
                                comments.push((counter, l.to_string()));
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
            file_name: file_name.to_string(),
            lines: comments,
            ..Default::default()
        })
    }

    fn get_comment_type(&self, line: &str) -> LuaCommentType {
        if contains_all(line, vec!["--[[", "]]"]) {
            return LuaCommentType::SingleLine;
        }

        if contains_any_of(line, vec!["]]"]) {
            return LuaCommentType::MultiLineEnd;
        }

        if contains_any_of(line, vec!["--[["]) {
            return LuaCommentType::MultiLineStart;
        }

        if contains_any_of(line, vec!["--"]) {
            return LuaCommentType::SingleLine;
        }

        LuaCommentType::None
    }
}

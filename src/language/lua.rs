use crate::language::{Comment, FindResult, Finder};
use crate::utils::path::file_name;
use crate::utils::string::contains_all;
use crate::utils::string::contains_any_of;

use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
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

    pub fn find(&self, path: PathBuf) -> io::Result<FindResult> {
        let mut counter = 1; // Lines begin on index 1
        let mut comments = Vec::<Comment>::new();
        let mut in_multiline = false;

        let file = File::open(&path)?;
        let file_name = file_name(&path)?;

        for line in BufReader::new(file).lines() {
            let content = line.expect("Could not read line");
            let comment_type = self.get_comment_type(&content);
            if in_multiline {
                // Ignore everything except MultiLineEnd when in a multiline-comment
                comments.push(Comment::new(counter, content.to_string()));
                in_multiline = comment_type != LuaCommentType::MultiLineEnd;
            } else {
                match self.get_comment_type(&content) {
                    LuaCommentType::SingleLine => {
                        comments.push(Comment::new(counter, content.to_string()))
                    }
                    LuaCommentType::MultiLineStart => {
                        in_multiline = true;
                        comments.push(Comment::new(counter, content.to_string()));
                    }
                    _ => (),
                }
            }
            counter += 1;
        }

        Ok(FindResult {
            file_name: file_name.to_string(),
            comments,
        })
    }

    fn get_comment_type(&self, comment: &str) -> LuaCommentType {
        if contains_all(comment, vec!["--[[", "]]"]) {
            return LuaCommentType::SingleLine;
        }

        if contains_any_of(comment, vec!["]]"]) {
            return LuaCommentType::MultiLineEnd;
        }

        if contains_any_of(comment, vec!["--[["]) {
            return LuaCommentType::MultiLineStart;
        }

        if contains_any_of(comment, vec!["--"]) {
            return LuaCommentType::SingleLine;
        }

        LuaCommentType::None
    }
}

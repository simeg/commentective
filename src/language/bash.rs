use crate::language::FindComment;
use crate::language::FindResult;
use crate::utils::comments::find_comments;
use crate::utils::comments::MultiCommentOpts;
use crate::utils::path::file_name;
use crate::utils::string::contains_any_of;
use crate::utils::string::str;

use std::fs::File;
use std::io::Error;
use std::path::PathBuf;

pub struct Bash {}

impl Default for Bash {
    fn default() -> Self {
        Self {}
    }
}

impl FindComment for Bash {
    fn find(&self, path: PathBuf) -> Result<FindResult, Error> {
        let file = File::open(&path)?;
        let file_name = file_name(&path)?;

        let multi_opts = MultiCommentOpts {
            starts: vec![str("<<COMMENT")],
            ends: vec![str("COMMENT")],
        };

        let comments = find_comments(&file, &multi_opts, &is_single_line_comment);

        Ok(FindResult {
            file_name,
            lines: comments,
            ..Default::default()
        })
    }
}

fn is_single_line_comment(line: &str) -> bool {
    contains_any_of(line, vec!["#"]) && !contains_any_of(line, vec!["#!/"])
}

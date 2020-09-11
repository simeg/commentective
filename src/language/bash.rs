use crate::language::FindResult;
use crate::language::Language;
use crate::utils::comments::find_comments;
use crate::utils::comments::MultiCommentOpts;
use crate::utils::path::file_name;
use crate::utils::string::contains_any_of;
use crate::utils::string::str;

use std::fs::File;
use std::io::Error;
use std::path::PathBuf;

pub struct Bash {
    pub path: PathBuf,
}

pub fn source(path: PathBuf) -> Bash {
    Bash { path }
}

fn multi_opts() -> MultiCommentOpts {
    MultiCommentOpts {
        starts: vec![str("<<COMMENT")],
        ends: vec![str("COMMENT")],
    }
}

impl Language for Bash {
    fn find(&self) -> Result<FindResult, Error> {
        let file = File::open(&self.path)?;
        let file_name = file_name(&self.path)?;

        let comments = find_comments(&file, &multi_opts(), &is_single_line_comment);

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

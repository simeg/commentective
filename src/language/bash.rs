use crate::language::{FindResult, OptsMultiComments, SimpleFindComments};
use crate::utils::string::contains_any_of;
use crate::utils::string::str;

use std::io::Error;
use std::path::PathBuf;

pub struct Bash<F: SimpleFindComments> {
    finder: F,
}

impl<F> Bash<F>
where
    F: SimpleFindComments,
{
    pub fn with_finder(finder: F) -> Self {
        Self { finder }
    }

    pub fn find(&self, path: PathBuf) -> Result<FindResult, Error> {
        let multi_opts = OptsMultiComments {
            starts: vec![str("<<COMMENT")],
            ends: vec![str("COMMENT")],
        };

        self.finder
            .find_comments(path, &multi_opts, is_single_line_comment)
    }
}

fn is_single_line_comment(line: &str) -> bool {
    contains_any_of(line, vec!["#"]) && !contains_any_of(line, vec!["#!/"])
}

use crate::language::{FindResult, OptsMultiComments, SimpleFindComments};
use crate::utils::string::contains_any_of;

use std::io;
use std::path::PathBuf;

pub struct Ruby<F: SimpleFindComments> {
    finder: F,
}

impl<F> Ruby<F>
where
    F: SimpleFindComments,
{
    pub fn with_finder(finder: F) -> Self {
        Self { finder }
    }

    pub fn find(&self, path: PathBuf) -> io::Result<FindResult> {
        let multi_opts = OptsMultiComments {
            starts: vec!["=begin".to_string()],
            ends: vec!["=end".to_string()],
        };

        self.finder
            .find_comments(path, &multi_opts, is_single_line_comment)
    }
}

fn is_single_line_comment(comment: &str) -> bool {
    contains_any_of(comment, vec!["#"])
}

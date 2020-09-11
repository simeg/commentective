use crate::language::FindResult;
use crate::language::Language;
use crate::utils::comments::find_comments;
use crate::utils::comments::MultiCommentOpts;
use crate::utils::path::filename;
use crate::utils::string::contains_all;
use crate::utils::string::str;
use std::fs::File;
use std::io::Error;
use std::io::ErrorKind;
use std::path::Path;

pub struct HTML {
    pub maybe_file: Result<File, Error>,
    pub file_name: String,
}

pub fn source(p: &Path) -> HTML {
    HTML {
        maybe_file: File::open(p),
        file_name: filename(p).unwrap(),
    }
}

fn multi_opts() -> MultiCommentOpts {
    MultiCommentOpts {
        starts: vec![str("<!--")],
        ends: vec![str("-->")],
    }
}

impl Language for HTML {
    fn find(&self) -> Result<FindResult, Error> {
        match self.maybe_file {
            Ok(ref file) => {
                let comments = find_comments(file, &multi_opts(), &is_single_line_comment);
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

fn is_single_line_comment(line: &str) -> bool {
    contains_all(line, vec!["<!--", "-->"])
}

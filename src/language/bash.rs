use language;
use language::FindResult;
use std::fs::File;
use std::io::Error;
use std::io::ErrorKind;
use std::path::Path;
use utils::comments::find_comments;
use utils::comments::MultiCommentOpts;
use utils::path::filename;
use utils::string::s;
use utils::string::string_contains_any_of;

#[derive(Debug)]
pub struct Bash {
    pub maybe_file: Result<File, Error>,
    pub file_name: String,
    pub multi_opts: MultiCommentOpts,
}

pub fn source(p: &Path) -> Bash {
    Bash {
        maybe_file: File::open(p),
        file_name: filename(p).unwrap(),
        multi_opts: multi_opts(),
    }
}

pub fn multi_opts() -> MultiCommentOpts {
    MultiCommentOpts {
        starts: vec![s("<<COMMENT")],
        ends: vec![s("COMMENT")],
    }
}

impl language::Language for Bash {
    #[inline]
    fn find(&self) -> Result<language::FindResult, Error> {
        match self.maybe_file {
            Ok(ref file) => {
                let comments = find_comments(file, &self.multi_opts, &is_single_line_comment);
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
    string_contains_any_of(s(line), vec!["#"]) && !string_contains_any_of(s(line), vec!["#!/"])
}

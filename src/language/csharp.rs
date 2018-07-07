use language;
use language::FindResult;
use std::fs::File;
use std::io::Error;
use std::io::ErrorKind;
use std::path::Path;
use std::result::Result::Err;
use utils::comments::find_comments;
use utils::path::filename;
use utils::string::s;
use utils::string::string_contains_any_of;

pub struct CSharp {
    pub maybe_file: Result<File, Error>,
    pub file_name: String,
}

pub fn source(p: &Path) -> CSharp {
    CSharp {
        maybe_file: File::open(p),
        file_name: filename(p).unwrap(),
    }
}

impl language::Language for CSharp {
    #[inline]
    fn find(&self) -> Result<language::FindResult, Error> {
        match self.maybe_file {
            Ok(ref file) => {
                let starts = vec![s("/*")];
                let ends = vec![s("*/")];
                let comments = find_comments(starts, ends, file, &is_single_line_comment);
                Ok(FindResult {
                    file_name: self.file_name.to_owned(),
                    lines: comments,
                })
            }
            Err(_) => Err(Error::new(ErrorKind::InvalidInput, "Could not parse file")),
        }
    }
}

fn is_single_line_comment(line: &str) -> bool {
    string_contains_any_of(String::from(line), vec!["//"])
}

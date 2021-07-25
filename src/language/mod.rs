use crate::utils::path::file_name;

use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub mod bash;
pub mod c;
pub mod cpp;
pub mod csharp;
pub mod css;
pub mod golang;
pub mod html;
pub mod java;
pub mod javascript;
pub mod lua;
pub mod php;
pub mod python;
pub mod ruby;
pub mod rust;
pub mod scala;

#[derive(Clone, Copy)]
pub struct Finder {}

pub trait SimpleFindComments {
    fn find_comments<F>(
        &self,
        path: PathBuf,
        multi_opts: &OptsMultiComments,
        is_single_line_comment: F,
    ) -> io::Result<FindResult>
    where
        F: Fn(&str) -> bool;
}

pub struct FindResult {
    pub file_name: String,
    pub comments: Vec<Comment>,
}

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct Comment {
    pub line_number: u32,
    pub content: String,
}

impl Comment {
    pub fn new(line_number: u32, content: String) -> Self {
        Self {
            line_number,
            content,
        }
    }
}

pub struct OptsMultiComments {
    pub starts: Vec<String>,
    pub ends: Vec<String>,
}

impl SimpleFindComments for Finder {
    fn find_comments<F>(
        &self,
        path: PathBuf,
        multi_opts: &OptsMultiComments,
        is_single_line_comment: F,
    ) -> io::Result<FindResult>
    where
        F: Fn(&str) -> bool,
    {
        let file = File::open(&path)?;
        let file_name = file_name(&path)?;

        let mut comments = Vec::<Comment>::new();
        let mut is_multi = false;

        for line in self.file_to_lines(&file) {
            let content = line.content.trim();
            let line_number = line.line_number;

            if is_single_line_comment(content) {
                comments.push(Comment::new(line_number, content.to_string()));
            } else {
                let is_multi_comment_start = self.is_in_list(content, multi_opts.starts.clone());
                let is_multi_comment_end = self.is_in_list(content, multi_opts.ends.clone());

                if is_multi_comment_start {
                    is_multi = true;
                    comments.push(Comment::new(line_number, content.to_string()));
                } else if is_multi_comment_end {
                    is_multi = false;
                    comments.push(Comment::new(line_number, content.to_string()));
                } else if is_multi {
                    comments.push(Comment::new(line_number, content.to_string()));
                }
            }
        }

        Ok(FindResult {
            file_name: file_name.to_string(),
            comments,
        })
    }
}

impl Finder {
    fn file_to_lines(&self, file: &File) -> Vec<Comment> {
        let mut counter: u32 = 1;
        BufReader::new(file)
            .lines()
            .map(|line| {
                let content = line.expect("Could not read line");
                let line = Comment {
                    line_number: counter,
                    content,
                };
                counter += 1;
                line
            })
            .collect()
    }

    fn is_in_list(&self, needle: &str, haystack: Vec<String>) -> bool {
        haystack
            .into_iter()
            .rfind(|ele| needle.contains(ele))
            .is_some()
    }
}

#[cfg(test)]
mod test {
    #![allow(non_snake_case)]

    use crate::language::{Comment, Finder};
    use std::collections::HashSet;
    use std::fs::File;
    use std::hash::Hash;
    use std::io::Write;

    type TestResult = Result<(), Box<dyn std::error::Error>>;

    #[test]
    fn finder__is_in_list__true() {
        let finder = Finder {};
        let needle = "needle";
        let haystack = vec!["arbitrary", "needle", "arbitrary-2"]
            .into_iter()
            .map(String::from)
            .collect();

        let actual = finder.is_in_list(needle, haystack);

        assert!(actual);
    }

    #[test]
    fn finder__is_in_list__false() {
        let finder = Finder {};
        let needle = "needle";
        let haystack = vec!["arbitrary", "arbitrary-2", "arbitrary-3"]
            .into_iter()
            .map(String::from)
            .collect();

        let actual = finder.is_in_list(needle, haystack);

        assert!(!actual);
    }

    #[test]
    fn finder__file_to_lines() -> TestResult {
        let finder = Finder {};
        let mut temp_file = tempfile::NamedTempFile::new()?;
        temp_file.write_all("line1\nline2\nline3\n".as_bytes())?;
        let file = File::open(temp_file.path());

        let actual = finder.file_to_lines(&file.unwrap());

        let expected = vec![
            Comment {
                line_number: 1,
                content: "line1".to_string(),
            },
            Comment {
                line_number: 2,
                content: "line2".to_string(),
            },
            Comment {
                line_number: 3,
                content: "line3".to_string(),
            },
        ];

        assert!(lists_equal(&actual, &expected));

        Ok(())
    }

    fn lists_equal<T>(a: &[T], b: &[T]) -> bool
    where
        T: Eq + Hash,
    {
        let a: HashSet<_> = a.iter().collect();
        let b: HashSet<_> = b.iter().collect();

        a == b
    }
}

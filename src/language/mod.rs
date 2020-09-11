use crate::utils::path::file_name;
use crate::utils::string::str;

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
    ) -> Result<FindResult, io::Error>
    where
        F: Fn(&str) -> bool;
}

pub struct FindResult {
    pub file_name: String,
    pub lines: Vec<u32>,
    pub print: bool,
}

impl Default for FindResult {
    fn default() -> FindResult {
        FindResult {
            file_name: str("DEFAULT_FILE_NAME"),
            lines: [].to_vec(),
            print: true,
        }
    }
}

#[derive(Hash, Eq, PartialEq)]
pub struct Line {
    pub index: u32,
    pub content: String,
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
    ) -> Result<FindResult, io::Error>
    where
        F: Fn(&str) -> bool,
    {
        let file = File::open(&path)?;
        let file_name = file_name(&path)?;

        let mut comment_lines = Vec::<u32>::new();
        let mut is_multi = false;

        for line in self.file_to_lines(&file) {
            let content = line.content.trim();
            let line_number = line.index;

            if is_single_line_comment(content) {
                comment_lines.push(line_number);
            } else {
                let is_multi_comment_start = self.in_list(content, multi_opts.starts.clone());
                let is_multi_comment_end = self.in_list(content, multi_opts.ends.clone());

                if is_multi_comment_start {
                    is_multi = true;
                    comment_lines.push(line_number);
                } else if is_multi_comment_end {
                    is_multi = false;
                    comment_lines.push(line_number);
                } else if is_multi {
                    comment_lines.push(line_number);
                }
            }
        }

        Ok(FindResult {
            file_name,
            lines: comment_lines,
            ..Default::default()
        })
    }
}

impl Finder {
    pub fn noop_find_result(&self) -> FindResult {
        FindResult {
            file_name: str("SHOULD_NOT_BE_PRINTED"),
            lines: [].to_vec(),
            print: false,
        }
    }

    fn file_to_lines(&self, file: &File) -> Vec<Line> {
        let mut counter: u32 = 1;
        BufReader::new(file)
            .lines()
            .map(|line| match line {
                Ok(content) => {
                    let line = Line {
                        index: counter,
                        content,
                    };
                    counter += 1;
                    line
                }
                Err(_) => panic!("Could not read line"),
            })
            .collect()
    }

    fn in_list(&self, needle: &str, haystack: Vec<String>) -> bool {
        haystack
            .into_iter()
            .rfind(|ele| needle.contains(ele))
            .is_some()
    }
}

#[cfg(test)]
mod test {
    #![allow(non_snake_case)]

    use crate::language::{Finder, Line};
    use std::collections::HashSet;
    use std::fs::File;
    use std::hash::Hash;
    use std::io::Write;

    type TestResult = Result<(), Box<dyn std::error::Error>>;

    #[test]
    fn test_comments__in_list__true() {
        let finder = Finder {};
        let needle = "needle";
        let haystack = vec!["arbitrary", "needle", "arbitrary-2"]
            .into_iter()
            .map(String::from)
            .collect();

        let actual = finder.in_list(needle, haystack);
        let expected = true;

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_comments__in_list__false() {
        let finder = Finder {};
        let needle = "needle";
        let haystack = vec!["arbitrary", "arbitrary-2", "arbitrary-3"]
            .into_iter()
            .map(String::from)
            .collect();

        let actual = finder.in_list(needle, haystack);
        let expected = false;

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_comments__file_to_lines() -> TestResult {
        let finder = Finder {};
        let mut temp_file = tempfile::NamedTempFile::new()?;
        temp_file.write_all("line1\nline2\nline3\n".as_bytes())?;
        let file = File::open(temp_file.path());

        let actual = finder.file_to_lines(&file.unwrap());

        let expected = vec![
            Line {
                index: 1,
                content: "line1".to_string(),
            },
            Line {
                index: 2,
                content: "line2".to_string(),
            },
            Line {
                index: 3,
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

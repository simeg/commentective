pub mod path {
    use crate::utils::string::str;
    use std::ffi::OsStr;
    use std::io;
    use std::io::Error;
    use std::io::ErrorKind;
    use std::path::Path;

    pub fn file_name(path: &Path) -> io::Result<String> {
        path.file_name()
            .map(OsStr::to_str)
            .flatten()
            .map(str)
            .ok_or_else(|| Error::new(ErrorKind::InvalidData, "Unable to get file name from path"))
    }

    pub fn extension(path: &Path) -> io::Result<String> {
        path.extension()
            .map(|oss| oss.to_str())
            .flatten()
            .map(str)
            .ok_or_else(|| Error::new(ErrorKind::InvalidData, "Unable to get extension from path"))
    }

    pub fn exists_on_filesystem(path: &OsStr) -> Result<(), String> {
        path.to_str()
            .map(Path::new)
            .map(Path::exists)
            .and_then(|exists| if exists { Some(Ok(())) } else { None })
            .unwrap_or_else(|| {
                Err(String::from(
                    "Unable to determine if file exists on file system",
                ))
            })
    }
}

pub mod string {
    pub fn contains_any_of(input: &str, matches: Vec<&str>) -> bool {
        matches
            .into_iter()
            .map(|pattern| input.contains(pattern))
            .any(|res| res)
    }

    pub fn contains_all(input: &str, matches: Vec<&str>) -> bool {
        let actual_matches = &matches.len();
        let found_matches = matches.into_iter().filter(|m| input.contains(m)).count();
        &found_matches == actual_matches
    }

    pub fn str(input: &str) -> String {
        String::from(input)
    }

    pub fn first_char(input: &str) -> char {
        input.chars().next().unwrap()
    }
}

pub mod comments {
    use crate::language::FindResult;
    use crate::utils::string::str;
    use std::fs::File;
    use std::io::BufRead;
    use std::io::BufReader;

    #[derive(Hash, Eq, PartialEq)]
    pub struct Line {
        pub index: u32,
        pub content: String,
    }

    pub struct MultiCommentOpts {
        pub starts: Vec<String>,
        pub ends: Vec<String>,
    }

    pub fn find_comments(
        file: &File,
        multi_opts: &MultiCommentOpts,
        is_single_line_comment: &dyn Fn(&str) -> bool,
    ) -> Vec<u32> {
        let mut comment_lines = Vec::<u32>::new();
        let mut is_multi = false;

        for line in file_to_lines(file) {
            let content = line.content.trim();
            let line_number = line.index;

            if is_single_line_comment(content) {
                comment_lines.push(line_number);
            } else {
                let is_multi_comment_start = in_list(content, multi_opts.starts.clone());
                let is_multi_comment_end = in_list(content, multi_opts.ends.clone());

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

        comment_lines
    }

    pub fn noop_find_result() -> FindResult {
        FindResult {
            file_name: str("SHOULD_NOT_BE_PRINTED"),
            lines: [].to_vec(),
            print: false,
        }
    }

    pub fn file_to_lines(file: &File) -> Vec<Line> {
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

    pub fn in_list(needle: &str, haystack: Vec<String>) -> bool {
        haystack
            .into_iter()
            .rfind(|ele| needle.contains(ele))
            .is_some()
    }
}

#[cfg(test)]
mod test {
    #![allow(non_snake_case)]

    use crate::utils::comments::{file_to_lines, in_list, Line};
    use crate::utils::path::{exists_on_filesystem, extension, file_name};
    use crate::utils::string::{contains_all, contains_any_of, first_char, str};
    use std::collections::HashSet;
    use std::fs::File;
    use std::hash::Hash;
    use std::io::Write;
    use std::path::Path;

    type TestResult = Result<(), Box<dyn std::error::Error>>;

    #[test]
    fn test_path__filename__ok() {
        let path = Path::new("dir/dir/some_file.js");

        let actual = file_name(path).unwrap();
        let expected = "some_file.js".to_string();

        assert_eq!(actual, expected)
    }

    #[test]
    fn test_path__filename__err() {
        let path = Path::new("");

        let actual = file_name(path);

        assert!(actual.is_err())
    }

    #[test]
    fn test_path__extension__ok() {
        let path = Path::new("dir/dir/some_file.js");

        let actual = extension(path).unwrap();
        let expected = "js".to_string();

        assert_eq!(actual, expected)
    }

    #[test]
    fn test_path__extension__err() {
        let path = Path::new("dir/dir/file_without_extension");

        let actual = extension(path);

        assert!(actual.is_err())
    }

    #[test]
    fn test_path__exists_on_filesystem__true() -> TestResult {
        let file = tempfile::NamedTempFile::new()?;

        let actual = exists_on_filesystem(file.path().as_ref());

        assert!(actual.is_ok());
        Ok(())
    }

    #[test]
    fn test_path__exists_on_filesystem__false() {
        let path = Path::new("/tmp/dev/null");

        let actual = exists_on_filesystem(path.as_ref());

        assert!(actual.is_err())
    }

    #[test]
    fn test_string__contains_all__true() {
        let input = "/* arbitrary */";
        let matches = vec!["/*", "*/"];

        let actual = contains_all(input, matches);
        let expected = true;

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_string__contains_all__false() {
        let input = "/* arbitrary */";
        let matches = vec!["/*", "not exist"];

        let actual = contains_all(input, matches);
        let expected = false;

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_string__contains_any_of__true() {
        let input = "/* arbitrary */";
        let matches = vec!["not exist", "*/"];

        let actual = contains_any_of(input, matches);
        let expected = true;

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_string__contains_any_of__false() {
        let input = "/* arbitrary */";
        let matches = vec!["not exist-1", "not exist-2"];

        let actual = contains_any_of(input, matches);
        let expected = false;

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_string__first_char() {
        let input = "abcde";

        let actual = first_char(input);
        let expected = 'a';

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_comments__in_list__true() {
        let needle = "needle";
        let haystack = vec!["arbitrary", "needle", "arbitrary-2"]
            .into_iter()
            .map(str)
            .collect();

        let actual = in_list(needle, haystack);
        let expected = true;

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_comments__in_list__false() {
        let needle = "needle";
        let haystack = vec!["arbitrary", "arbitrary-2", "arbitrary-3"]
            .into_iter()
            .map(str)
            .collect();

        let actual = in_list(needle, haystack);
        let expected = false;

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_comments__file_to_lines() -> TestResult {
        let mut temp_file = tempfile::NamedTempFile::new()?;
        temp_file.write_all("line1\nline2\nline3\n".as_bytes())?;
        let file = File::open(temp_file.path());

        let actual = file_to_lines(&file.unwrap());

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

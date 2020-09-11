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

#[cfg(test)]
mod test {
    #![allow(non_snake_case)]

    use crate::utils::path::{exists_on_filesystem, extension, file_name};
    use crate::utils::string::{contains_all, contains_any_of, first_char};
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
}

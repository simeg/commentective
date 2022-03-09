pub mod path {
    use std::ffi::OsStr;
    use std::fs::metadata;
    use std::io;
    use std::io::{Error, ErrorKind};
    use std::path::Path;

    pub fn file_name(path: &Path) -> io::Result<&str> {
        path.file_name()
            .and_then(OsStr::to_str)
            .ok_or_else(|| Error::new(ErrorKind::InvalidData, "Unable to get file name from path"))
    }

    pub fn exists_on_filesystem(path: &OsStr) -> Result<(), String> {
        match Some(path).map(Path::new).map(Path::exists).unwrap_or(false) {
            true => Ok(()),
            false => Err(format!("File not found [{:?}]", path)),
        }
    }

    pub fn is_file(path: &OsStr) -> Result<(), String> {
        match metadata(path)
            .map(|metadata| metadata.is_file())
            .unwrap_or(false)
        {
            true => Ok(()),
            false => Err(format!("Not a file [{:?}]", path)),
        }
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

    pub fn first_char(input: &str) -> char {
        input.chars().next().unwrap()
    }
}

#[cfg(test)]
mod test {
    #![allow(non_snake_case)]

    use crate::utils::path::{exists_on_filesystem, file_name, is_file};
    use crate::utils::string::{contains_all, contains_any_of, first_char};
    use std::path::Path;
    use tempfile::tempdir;

    type TestResult = Result<(), Box<dyn std::error::Error>>;

    #[test]
    fn path__file_name__ok() {
        let path = Path::new("dir/dir/some_file.js");

        let actual = file_name(path).unwrap();
        let expected = "some_file.js".to_string();

        assert_eq!(actual, expected)
    }

    #[test]
    fn path__file_name__err() {
        let path = Path::new("");

        let actual = file_name(path);

        assert!(actual.is_err())
    }

    #[test]
    fn path__exists_on_filesystem__true() -> TestResult {
        let file = tempfile::NamedTempFile::new()?;

        let actual = exists_on_filesystem(file.path().as_ref());

        assert!(actual.is_ok());
        Ok(())
    }

    #[test]
    fn path__exists_on_filesystem__false() {
        let path = Path::new("/tmp/dev/null");

        let actual = exists_on_filesystem(path.as_ref());

        assert!(actual.is_err())
    }

    #[test]
    fn path__is_file__true() -> TestResult {
        let file = tempfile::NamedTempFile::new()?;

        let actual = is_file(file.path().as_os_str());

        assert!(actual.is_ok());
        Ok(())
    }

    #[test]
    fn path__is_file__false() -> TestResult {
        let not_file = tempdir()?;

        let actual = is_file(not_file.path().as_os_str());

        assert!(actual.is_err());
        Ok(())
    }

    #[test]
    fn string__contains_all__true() {
        let input = "/* arbitrary */";
        let matches = vec!["/*", "*/"];

        let actual = contains_all(input, matches);
        let expected = true;

        assert_eq!(actual, expected);
    }

    #[test]
    fn string__contains_all__false() {
        let input = "/* arbitrary */";
        let matches = vec!["/*", "not exist"];

        let actual = contains_all(input, matches);
        let expected = false;

        assert_eq!(actual, expected);
    }

    #[test]
    fn string__contains_any_of__true() {
        let input = "/* arbitrary */";
        let matches = vec!["not exist", "*/"];

        let actual = contains_any_of(input, matches);
        let expected = true;

        assert_eq!(actual, expected);
    }

    #[test]
    fn string__contains_any_of__false() {
        let input = "/* arbitrary */";
        let matches = vec!["not exist-1", "not exist-2"];

        let actual = contains_any_of(input, matches);
        let expected = false;

        assert_eq!(actual, expected);
    }

    #[test]
    fn string__first_char() {
        let input = "abcde";

        let actual = first_char(input);
        let expected = 'a';

        assert_eq!(actual, expected);
    }
}

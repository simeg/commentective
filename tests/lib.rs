extern crate detector;

#[cfg(test)]
mod tests {
    use detector;
    use detector::language as l;
    use detector::language::Language;
    use detector::utils;
    use std::ffi::OsStr;
    use std::fs::File;
    use std::path::Path;

    const NON_EXISTING_FILE: &'static str = "I_DO_NOT_EXIST";
    const EMPTY_FILE: &'static str = "tests/resources/empty.foo";
    const UNSUPPORTED_FILE: &'static str = "tests/resources/empty.foo";

    #[test]
    fn js_find_with_value() {
        let path = Path::new("tests/resources/js-with-comments.js");
        let result = l::javascript::Js {
            maybe_file: File::open(path),
            file_name: String::from("irrelevant-name"),
        }.find();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().lines.len(), 8);
    }

    #[test]
    fn js_find_with_err() {
        let path = Path::new("tests/resources/js-without-comments.js");
        let result = l::javascript::Js {
            maybe_file: File::open(path),
            file_name: String::from("irrelevant-name"),
        }.find();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().lines.len(), 0);
    }

    #[test]
    fn java_find_with_value() {
        let path = Path::new("tests/resources/java-with-comments.java");
        let result = l::java::Java {
            maybe_file: File::open(path),
            file_name: String::from("irrelevant-name"),
        }.find();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().lines.len(), 7);
    }

    #[test]
    fn java_find_with_err() {
        let path = Path::new("tests/resources/java-without-comments.java");
        let result = l::java::Java {
            maybe_file: File::open(path),
            file_name: String::from("irrelevant-name"),
        }.find();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().lines.len(), 0);
    }

    #[test]
    fn resolve_type_with_value() {
        let path = Path::new("tests/resources/js-with-comments.js");
        let result = detector::resolve_type(path);
        assert!(result.is_ok());
    }

    #[test]
    fn resolve_type_with_err() {
        let path = Path::new(UNSUPPORTED_FILE);
        let result = detector::resolve_type(path);
        assert!(result.is_err());
    }

    #[test]
    fn exists_on_filesystem_with_value() {
        let path = OsStr::new(EMPTY_FILE);
        let result = detector::exists_on_filesystem(path);
        assert!(result.is_ok());
    }

    #[test]
    fn exists_on_filesystem_with_err() {
        let path = OsStr::new(NON_EXISTING_FILE);
        let result = detector::exists_on_filesystem(path);
        assert!(result.is_err());
    }

    #[test]
    fn utils_path_filename_with_value() {
        let path = Path::new(EMPTY_FILE);
        let result = utils::path::filename(path);
        assert!(result.is_ok());
    }

    #[test]
    fn utils_path_filename_with_err() {
        let path = Path::new(NON_EXISTING_FILE);
        let result = utils::path::filename(path);
        assert!(result.is_ok());
    }
}

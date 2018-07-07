extern crate detector;

#[cfg(test)]
mod tests {
    use detector::language as l;
    use detector::language::bash::Bash;
    use detector::language::csharp::CSharp;
    use detector::language::java::Java;
    use detector::language::javascript::Js;
    use detector::language::php::PHP;
    use detector::language::python::Python;
    use detector::language::rust::Rust;
    use detector::language::Language;
    use detector::utils::string::s;
    use std::fs::File;
    use std::path::Path;

    #[test]
    fn javascript_find_with_value() {
        let path = Path::new("tests/resources/js/with-comments.js");
        let result = Js {
            maybe_file: File::open(path),
            file_name: s("irrelevant-name"),
            multi_opts: l::javascript::multi_opts(),
        }.find();
        assert!(result.is_ok());
        let lines = result.unwrap().lines;
        assert_eq!(lines.len(), 11);
        assert_eq!(lines, [1, 3, 4, 5, 7, 8, 9, 11, 12, 13, 15]);
    }

    #[test]
    fn javascript_find_with_err() {
        let path = Path::new("tests/resources/js/without-comments.js");
        let result = Js {
            maybe_file: File::open(path),
            file_name: s("irrelevant-name"),
            multi_opts: l::javascript::multi_opts(),
        }.find();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().lines.len(), 0);
    }

    #[test]
    fn java_find_with_value() {
        let path = Path::new("tests/resources/java/with-comments.java");
        let result = Java {
            maybe_file: File::open(path),
            file_name: s("irrelevant-name"),
            multi_opts: l::java::multi_opts(),
        }.find();
        assert!(result.is_ok());
        let lines = result.unwrap().lines;
        assert_eq!(lines.len(), 15);
        assert_eq!(
            lines,
            [5, 6, 7, 9, 11, 12, 13, 15, 16, 17, 19, 20, 21, 23, 25]
        );
    }

    #[test]
    fn java_find_with_err() {
        let path = Path::new("tests/resources/java/without-comments.java");
        let result = Java {
            maybe_file: File::open(path),
            file_name: s("irrelevant-name"),
            multi_opts: l::java::multi_opts(),
        }.find();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().lines.len(), 0);
    }

    #[test]
    fn rust_find_with_value() {
        let path = Path::new("tests/resources/rust/with-comments.rs");
        let result = Rust {
            maybe_file: File::open(path),
            file_name: s("irrelevant-name"),
            multi_opts: l::rust::multi_opts(),
        }.find();
        assert!(result.is_ok());
        let lines = result.unwrap().lines;
        assert_eq!(lines.len(), 8);
        assert_eq!(lines, [1, 4, 5, 6, 9, 10, 11, 13]);
    }

    #[test]
    fn rust_find_with_err() {
        let path = Path::new("tests/resources/rust/without-comments.rs");
        let result = Rust {
            maybe_file: File::open(path),
            file_name: s("irrelevant-name"),
            multi_opts: l::rust::multi_opts(),
        }.find();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().lines.len(), 0);
    }

    #[test]
    fn python_find_with_value() {
        let path = Path::new("tests/resources/python/with-comments.py");
        let result = Python {
            maybe_file: File::open(path),
            file_name: s("irrelevant-name"),
        }.find();
        assert!(result.is_ok());
        let lines = result.unwrap().lines;
        assert_eq!(lines.len(), 4);
        assert_eq!(lines, [1, 3, 4, 5]);
    }

    #[test]
    fn python_find_with_err() {
        let path = Path::new("tests/resources/python/without-comments.py");
        let result = Python {
            maybe_file: File::open(path),
            file_name: s("irrelevant-name"),
        }.find();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().lines.len(), 0);
    }

    #[test]
    fn csharp_find_with_value() {
        let path = Path::new("tests/resources/csharp/with-comments.cs");
        let result = CSharp {
            maybe_file: File::open(path),
            file_name: s("irrelevant-name"),
            multi_opts: l::csharp::multi_opts(),
        }.find();
        assert!(result.is_ok());
        let lines = result.unwrap().lines;
        assert_eq!(lines.len(), 7);
        assert_eq!(lines, [1, 3, 5, 6, 7, 8, 10])
    }

    #[test]
    fn csharp_find_with_err() {
        let path = Path::new("tests/resources/csharp/without-comments.cs");
        let result = CSharp {
            maybe_file: File::open(path),
            file_name: s("irrelevant-name"),
            multi_opts: l::csharp::multi_opts(),
        }.find();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().lines.len(), 0);
    }

    #[test]
    fn bash_find_with_value() {
        let path = Path::new("tests/resources/bash/with-comments.sh");
        let result = Bash {
            maybe_file: File::open(path),
            file_name: s("irrelevant-name"),
            multi_opts: l::bash::multi_opts(),
        }.find();
        assert!(result.is_ok());
        let lines = result.unwrap().lines;
        assert_eq!(lines.len(), 5);
        assert_eq!(lines, [3, 6, 7, 8, 9]);
    }

    #[test]
    fn bash_find_with_err() {
        let path = Path::new("tests/resources/bash/without-comments.sh");
        let result = Bash {
            maybe_file: File::open(path),
            file_name: s("irrelevant-name"),
            multi_opts: l::bash::multi_opts(),
        }.find();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().lines.len(), 0);
    }

    #[test]
    fn php_find_with_value() {
        let path = Path::new("tests/resources/php/with-comments.php");
        let result = PHP {
            maybe_file: File::open(path),
            file_name: s("irrelevant-name"),
            multi_opts: l::php::multi_opts(),
        }.find();
        assert!(result.is_ok());
        let lines = result.unwrap().lines;
        assert_eq!(lines.len(), 9);
        assert_eq!(lines, [3, 5, 7, 9, 10, 11, 13, 14, 15]);
    }

    #[test]
    fn php_find_with_err() {
        let path = Path::new("tests/resources/php/without-comments.php");
        let result = PHP {
            maybe_file: File::open(path),
            file_name: s("irrelevant-name"),
            multi_opts: l::php::multi_opts(),
        }.find();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().lines.len(), 0);
    }
}

#[cfg(test)]
mod utils {
    use detector;
    use detector::utils;
    use detector::utils::path::exists_on_filesystem;
    use detector::utils::string::s;
    use std::ffi::OsStr;
    use std::path::Path;

    const NON_EXISTING_FILE: &'static str = "I_DO_NOT_EXIST";
    const EXISTING_FILE: &'static str = "tests/resources/js/with-comments.js";
    const EMPTY_FILE: &'static str = "tests/resources/empty.foo";
    const UNSUPPORTED_FILE: &'static str = "tests/resources/empty.foo";

    #[test]
    fn resolve_type_with_value() {
        let path = Path::new(EXISTING_FILE);
        let result = detector::resolve_type_and_run(path);
        assert!(result.is_ok());
    }

    #[test]
    fn resolve_type_with_err() {
        let path = Path::new(UNSUPPORTED_FILE);
        let result = detector::resolve_type_and_run(path);
        assert!(result.is_err());
    }

    #[test]
    fn exists_on_filesystem_with_value() {
        let path = OsStr::new(EMPTY_FILE);
        let result = exists_on_filesystem(path);
        assert!(result.is_ok());
    }

    #[test]
    fn exists_on_filesystem_with_err() {
        let path = OsStr::new(NON_EXISTING_FILE);
        let result = exists_on_filesystem(path);
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

    #[test]
    fn utils_string_contains_all_true() {
        let result = utils::string::string_contains_all(s("/* arbitrary */"), vec!["/*", "*/"]);
        assert!(result);
    }

    #[test]
    fn utils_string_contains_all_false() {
        let result =
            utils::string::string_contains_all(s("/* arbitrary */"), vec!["/*", "not exist"]);
        assert!(!result);
    }
}

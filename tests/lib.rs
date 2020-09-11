#[cfg(test)]
mod languages {
    #![allow(non_snake_case)]

    use commentective::language::bash::Bash;
    use commentective::language::c::C;
    use commentective::language::cpp::Cpp;
    use commentective::language::csharp::CSharp;
    use commentective::language::css::CSS;
    use commentective::language::golang::Go;
    use commentective::language::html::HTML;
    use commentective::language::java::Java;
    use commentective::language::javascript::JavaScript;
    use commentective::language::lua::Lua;
    use commentective::language::php::PHP;
    use commentective::language::python::Python;
    use commentective::language::ruby::Ruby;
    use commentective::language::rust::Rust;
    use commentective::language::scala::Scala;
    use commentective::language::FindComment;
    use std::path::PathBuf;

    #[test]
    fn javascript__find__with_comments() {
        let path = PathBuf::from("tests/resources/javascript/with-comments.js");
        let result = JavaScript::default().find(path);
        assert!(result.is_ok());
        let lines = result.unwrap().lines;
        assert_eq!(lines.len(), 13);
        assert_eq!(lines, [1, 3, 4, 5, 7, 8, 9, 11, 12, 13, 15, 17, 18]);
    }

    #[test]
    fn javascript__find__without_comments() {
        let path = PathBuf::from("tests/resources/javascript/without-comments.js");
        let result = JavaScript::default().find(path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().lines.len(), 0);
    }

    #[test]
    fn java__find__with_comments() {
        let path = PathBuf::from("tests/resources/java/with-comments.java");
        let result = Java::default().find(path);
        assert!(result.is_ok());
        let lines = result.unwrap().lines;
        assert_eq!(lines.len(), 17);
        assert_eq!(
            lines,
            [5, 6, 7, 9, 11, 12, 13, 15, 16, 17, 19, 20, 21, 23, 25, 27, 28]
        );
    }

    #[test]
    fn java__find__without_comments() {
        let path = PathBuf::from("tests/resources/java/without-comments.java");
        let result = Java::default().find(path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().lines.len(), 0);
    }

    #[test]
    fn rust__find__with_comments() {
        let path = PathBuf::from("tests/resources/rust/with-comments.rs");
        let result = Rust::default().find(path);
        assert!(result.is_ok());
        let lines = result.unwrap().lines;
        assert_eq!(lines.len(), 10);
        assert_eq!(lines, [1, 4, 5, 6, 9, 10, 11, 13, 15, 16]);
    }

    #[test]
    fn rust__find__without_comments() {
        let path = PathBuf::from("tests/resources/rust/without-comments.rs");
        let result = Rust::default().find(path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().lines.len(), 0);
    }

    #[test]
    fn python__find__with_comments() {
        let path = PathBuf::from("tests/resources/python/with-comments.py");
        let result = Python::default().find(path);
        assert!(result.is_ok());
        let lines = result.unwrap().lines;
        assert_eq!(lines.len(), 4);
        assert_eq!(lines, [1, 3, 4, 5]);
    }

    #[test]
    fn python__find__without_comments() {
        let path = PathBuf::from("tests/resources/python/without-comments.py");
        let result = Python::default().find(path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().lines.len(), 0);
    }

    #[test]
    fn csharp__find__with_comments() {
        let path = PathBuf::from("tests/resources/csharp/with-comments.cs");
        let result = CSharp::default().find(path);
        assert!(result.is_ok());
        let lines = result.unwrap().lines;
        assert_eq!(lines.len(), 9);
        assert_eq!(lines, [1, 3, 5, 6, 7, 8, 10, 12, 13])
    }

    #[test]
    fn csharp__find__without_comments() {
        let path = PathBuf::from("tests/resources/csharp/without-comments.cs");
        let result = CSharp::default().find(path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().lines.len(), 0);
    }

    #[test]
    fn bash__find__with_comments() {
        let path = PathBuf::from("tests/resources/bash/with-comments.sh");
        let result = Bash::default().find(path);
        assert!(result.is_ok());
        let lines = result.unwrap().lines;
        assert_eq!(lines.len(), 5);
        assert_eq!(lines, [3, 6, 7, 8, 9]);
    }

    #[test]
    fn bash__find__without_comments() {
        let path = PathBuf::from("tests/resources/bash/without-comments.sh");
        let result = Bash::default().find(path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().lines.len(), 0);
    }

    #[test]
    fn php__find__with_comments() {
        let path = PathBuf::from("tests/resources/php/with-comments.php");
        let result = PHP::default().find(path);
        assert!(result.is_ok());
        let lines = result.unwrap().lines;
        assert_eq!(lines.len(), 11);
        assert_eq!(lines, [3, 5, 7, 9, 10, 11, 13, 14, 15, 17, 18]);
    }

    #[test]
    fn php__find__without_comments() {
        let path = PathBuf::from("tests/resources/php/without-comments.php");
        let result = PHP::default().find(path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().lines.len(), 0);
    }

    #[test]
    fn ruby__find__with_comments() {
        let path = PathBuf::from("tests/resources/ruby/with-comments.rb");
        let result = Ruby::default().find(path);
        assert!(result.is_ok());
        let lines = result.unwrap().lines;
        assert_eq!(lines.len(), 6);
        assert_eq!(lines, [1, 2, 3, 5, 8, 9]);
    }

    #[test]
    fn ruby__find__without_comments() {
        let path = PathBuf::from("tests/resources/ruby/without-comments.rb");
        let result = Ruby::default().find(path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().lines.len(), 0);
    }

    #[test]
    fn golang__find__with_comments() {
        let path = PathBuf::from("tests/resources/golang/with-comments.go");
        let result = Go::default().find(path);
        assert!(result.is_ok());
        let lines = result.unwrap().lines;
        assert_eq!(lines.len(), 2);
        assert_eq!(lines, [1, 2]);
    }

    #[test]
    fn golang__find__without_comments() {
        let path = PathBuf::from("tests/resources/golang/without-comments.go");
        let result = Go::default().find(path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().lines.len(), 0);
    }

    #[test]
    fn scala__find__with_comments() {
        let path = PathBuf::from("tests/resources/scala/with-comments.scala");
        let result = Scala::default().find(path);
        assert!(result.is_ok());
        let lines = result.unwrap().lines;
        assert_eq!(lines.len(), 11);
        assert_eq!(lines, [1, 7, 8, 9, 11, 12, 13, 15, 16, 18, 19]);
    }

    #[test]
    fn scala__find__without_comments() {
        let path = PathBuf::from("tests/resources/scala/without-comments.scala");
        let result = Scala::default().find(path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().lines.len(), 0);
    }

    #[test]
    fn css__find__with_comments() {
        let path = PathBuf::from("tests/resources/css/with-comments.css");
        let result = CSS::default().find(path);
        assert!(result.is_ok());
        let lines = result.unwrap().lines;
        assert_eq!(lines.len(), 6);
        assert_eq!(lines, [1, 3, 6, 7, 8, 10]);
    }

    #[test]
    fn css__find__without_comments() {
        let path = PathBuf::from("tests/resources/css/without-comments.css");
        let result = CSS::default().find(path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().lines.len(), 0);
    }

    #[test]
    fn html__find__with_comments() {
        let path = PathBuf::from("tests/resources/html/with-comments.html");
        let result = HTML::default().find(path);
        assert!(result.is_ok());
        let lines = result.unwrap().lines;
        assert_eq!(lines.len(), 6);
        assert_eq!(lines, [1, 3, 5, 7, 8, 9]);
    }

    #[test]
    fn html__find__without_comments() {
        let path = PathBuf::from("tests/resources/html/without-comments.html");
        let result = HTML::default().find(path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().lines.len(), 0);
    }

    #[test]
    fn c__find__with_comments() {
        let path = PathBuf::from("tests/resources/c/with-comments.c");
        let result = C::default().find(path);
        assert!(result.is_ok());
        let lines = result.unwrap().lines;
        assert_eq!(lines.len(), 8);
        assert_eq!(lines, [1, 3, 5, 6, 7, 9, 10, 12]);
    }

    #[test]
    fn c__find__without_comments() {
        let path = PathBuf::from("tests/resources/c/without-comments.c");
        let result = C::default().find(path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().lines.len(), 0);
    }

    #[test]
    fn cpp__find__with_comments() {
        let path = PathBuf::from("tests/resources/cpp/with-comments.cpp");
        let result = Cpp::default().find(path);
        assert!(result.is_ok());
        let lines = result.unwrap().lines;
        assert_eq!(lines.len(), 11);
        assert_eq!(lines, [1, 3, 5, 6, 7, 9, 10, 12, 14, 15, 16]);
    }

    #[test]
    fn cpp__find__without_comments() {
        let path = PathBuf::from("tests/resources/cpp/without-comments.cpp");
        let result = Cpp::default().find(path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().lines.len(), 0);
    }

    #[test]
    fn lua__find__with_comments() {
        let path = PathBuf::from("tests/resources/lua/with-comments.lua");
        let result = Lua::default().find(path);
        assert!(result.is_ok());
        let lines = result.unwrap().lines;
        assert_eq!(lines.len(), 13);
        assert_eq!(lines, [1, 2, 5, 6, 7, 9, 10, 11, 13, 14, 15, 19, 21]);
    }

    #[test]
    fn lua__find__without_comments() {
        let path = PathBuf::from("tests/resources/lua/without-comments.lua");
        let result = Lua::default().find(path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().lines.len(), 0);
    }
}

#[cfg(test)]
mod flags {
    use commentective::OptsCli;
    use std::path::PathBuf;

    #[test]
    fn flag_extension() {
        let path_javascript = PathBuf::from("tests/resources/javascript/with-comments.js");
        let path_python = PathBuf::from("tests/resources/python/with-comments.py");
        let paths = vec![path_javascript, path_python];

        let opts_no_ext = OptsCli {
            extension: None,
            short: false,
            ignore_empty: false,
        };
        for (i, result) in commentective::run(paths.clone(), &opts_no_ext)
            .iter()
            .enumerate()
        {
            assert!(result.is_ok());

            if i == 0 {
                // JavaScript
                result
                    .iter()
                    .map(|res| assert_eq!(res.lines.len(), 13))
                    .for_each(drop);
            } else {
                // Python
                result
                    .iter()
                    .map(|res| assert_eq!(res.lines.len(), 4))
                    .for_each(drop);
            }
        }

        let opts_with_ext = OptsCli {
            extension: Some("js".to_string()),
            short: false,
            ignore_empty: false,
        };
        for (i, result) in commentective::run(paths, &opts_with_ext).iter().enumerate() {
            assert!(result.is_ok());

            if i == 0 {
                // JavaScript
                result
                    .iter()
                    .map(|res| assert_eq!(res.lines.len(), 13))
                    .for_each(drop);
            } else {
                // Python
                result
                    .iter()
                    .map(|res| assert_eq!(res.lines.len(), 0))
                    .for_each(drop);
            }
        }
    }
}

#[cfg(test)]
mod utils {
    #![allow(non_snake_case)]

    use commentective::OptsCli;
    use std::path::PathBuf;

    const EXISTING_FILE: &'static str = "tests/resources/javascript/with-comments.js";
    const UNSUPPORTED_FILE: &'static str = "tests/resources/empty.foo";

    #[test]
    fn resolve_type__with_comments() {
        let path = PathBuf::from(EXISTING_FILE);
        let result = commentective::resolve_type_and_run(
            path,
            &OptsCli {
                extension: None,
                short: false,
                ignore_empty: false,
            },
        );
        assert!(result.is_ok());
    }

    #[test]
    fn resolve_type__without_comments() {
        let path = PathBuf::from(UNSUPPORTED_FILE);
        let result = commentective::resolve_type_and_run(
            path,
            &OptsCli {
                extension: None,
                short: false,
                ignore_empty: false,
            },
        );
        assert!(result.is_err());
    }
}

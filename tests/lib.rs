#[cfg(test)]
mod languages {
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
    use commentective::language::{
        bash, c, cpp, csharp, css, html, java, javascript, php, ruby, rust, scala, Language,
    };
    use commentective::utils::string::str;
    use std::fs::File;
    use std::path::Path;

    const IRRELEVANT: &'static str = "irrelevant";

    #[test]
    fn javascript_find_with_value() {
        let path = Path::new("tests/resources/javascript/with-comments.js");
        let result = JavaScript {
            maybe_file: File::open(path),
            file_name: str(IRRELEVANT),
            multi_opts: javascript::multi_opts(),
        }
        .find();
        assert!(result.is_ok());
        let lines = result.unwrap().lines;
        assert_eq!(lines.len(), 13);
        assert_eq!(lines, [1, 3, 4, 5, 7, 8, 9, 11, 12, 13, 15, 17, 18]);
    }

    #[test]
    fn javascript_find_with_err() {
        let path = Path::new("tests/resources/javascript/without-comments.js");
        let result = JavaScript {
            maybe_file: File::open(path),
            file_name: str(IRRELEVANT),
            multi_opts: javascript::multi_opts(),
        }
        .find();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().lines.len(), 0);
    }

    #[test]
    fn java_find_with_value() {
        let path = Path::new("tests/resources/java/with-comments.java");
        let result = Java {
            maybe_file: File::open(path),
            file_name: str(IRRELEVANT),
            multi_opts: java::multi_opts(),
        }
        .find();
        assert!(result.is_ok());
        let lines = result.unwrap().lines;
        assert_eq!(lines.len(), 17);
        assert_eq!(
            lines,
            [5, 6, 7, 9, 11, 12, 13, 15, 16, 17, 19, 20, 21, 23, 25, 27, 28]
        );
    }

    #[test]
    fn java_find_with_err() {
        let path = Path::new("tests/resources/java/without-comments.java");
        let result = Java {
            maybe_file: File::open(path),
            file_name: str(IRRELEVANT),
            multi_opts: java::multi_opts(),
        }
        .find();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().lines.len(), 0);
    }

    #[test]
    fn rust_find_with_value() {
        let path = Path::new("tests/resources/rust/with-comments.rs");
        let result = Rust {
            maybe_file: File::open(path),
            file_name: str(IRRELEVANT),
            multi_opts: rust::multi_opts(),
        }
        .find();
        assert!(result.is_ok());
        let lines = result.unwrap().lines;
        assert_eq!(lines.len(), 10);
        assert_eq!(lines, [1, 4, 5, 6, 9, 10, 11, 13, 15, 16]);
    }

    #[test]
    fn rust_find_with_err() {
        let path = Path::new("tests/resources/rust/without-comments.rs");
        let result = Rust {
            maybe_file: File::open(path),
            file_name: str(IRRELEVANT),
            multi_opts: rust::multi_opts(),
        }
        .find();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().lines.len(), 0);
    }

    #[test]
    fn python_find_with_value() {
        let path = Path::new("tests/resources/python/with-comments.py");
        let result = Python {
            maybe_file: File::open(path),
            file_name: str(IRRELEVANT),
        }
        .find();
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
            file_name: str(IRRELEVANT),
        }
        .find();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().lines.len(), 0);
    }

    #[test]
    fn csharp_find_with_value() {
        let path = Path::new("tests/resources/csharp/with-comments.cs");
        let result = CSharp {
            maybe_file: File::open(path),
            file_name: str(IRRELEVANT),
            multi_opts: csharp::multi_opts(),
        }
        .find();
        assert!(result.is_ok());
        let lines = result.unwrap().lines;
        assert_eq!(lines.len(), 9);
        assert_eq!(lines, [1, 3, 5, 6, 7, 8, 10, 12, 13])
    }

    #[test]
    fn csharp_find_with_err() {
        let path = Path::new("tests/resources/csharp/without-comments.cs");
        let result = CSharp {
            maybe_file: File::open(path),
            file_name: str(IRRELEVANT),
            multi_opts: csharp::multi_opts(),
        }
        .find();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().lines.len(), 0);
    }

    #[test]
    fn bash_find_with_value() {
        let path = Path::new("tests/resources/bash/with-comments.sh");
        let result = Bash {
            maybe_file: File::open(path),
            file_name: str(IRRELEVANT),
            multi_opts: bash::multi_opts(),
        }
        .find();
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
            file_name: str(IRRELEVANT),
            multi_opts: bash::multi_opts(),
        }
        .find();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().lines.len(), 0);
    }

    #[test]
    fn php_find_with_value() {
        let path = Path::new("tests/resources/php/with-comments.php");
        let result = PHP {
            maybe_file: File::open(path),
            file_name: str(IRRELEVANT),
            multi_opts: php::multi_opts(),
        }
        .find();
        assert!(result.is_ok());
        let lines = result.unwrap().lines;
        assert_eq!(lines.len(), 11);
        assert_eq!(lines, [3, 5, 7, 9, 10, 11, 13, 14, 15, 17, 18]);
    }

    #[test]
    fn php_find_with_err() {
        let path = Path::new("tests/resources/php/without-comments.php");
        let result = PHP {
            maybe_file: File::open(path),
            file_name: str(IRRELEVANT),
            multi_opts: php::multi_opts(),
        }
        .find();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().lines.len(), 0);
    }

    #[test]
    fn ruby_find_with_value() {
        let path = Path::new("tests/resources/ruby/with-comments.rb");
        let result = Ruby {
            maybe_file: File::open(path),
            file_name: str(IRRELEVANT),
            multi_opts: ruby::multi_opts(),
        }
        .find();
        assert!(result.is_ok());
        let lines = result.unwrap().lines;
        assert_eq!(lines.len(), 6);
        assert_eq!(lines, [1, 2, 3, 5, 8, 9]);
    }

    #[test]
    fn ruby_find_with_err() {
        let path = Path::new("tests/resources/ruby/without-comments.rb");
        let result = Ruby {
            maybe_file: File::open(path),
            file_name: str(IRRELEVANT),
            multi_opts: ruby::multi_opts(),
        }
        .find();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().lines.len(), 0);
    }

    #[test]
    fn golang_find_with_value() {
        let path = Path::new("tests/resources/golang/with-comments.go");
        let result = Go {
            maybe_file: File::open(path),
            file_name: str(IRRELEVANT),
        }
        .find();
        assert!(result.is_ok());
        let lines = result.unwrap().lines;
        assert_eq!(lines.len(), 2);
        assert_eq!(lines, [1, 2]);
    }

    #[test]
    fn golang_find_with_err() {
        let path = Path::new("tests/resources/golang/without-comments.go");
        let result = Go {
            maybe_file: File::open(path),
            file_name: str(IRRELEVANT),
        }
        .find();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().lines.len(), 0);
    }

    #[test]
    fn scala_find_with_value() {
        let path = Path::new("tests/resources/scala/with-comments.scala");
        let result = Scala {
            maybe_file: File::open(path),
            file_name: str(IRRELEVANT),
            multi_opts: scala::multi_opts(),
        }
        .find();
        assert!(result.is_ok());
        let lines = result.unwrap().lines;
        assert_eq!(lines.len(), 11);
        assert_eq!(lines, [1, 7, 8, 9, 11, 12, 13, 15, 16, 18, 19]);
    }

    #[test]
    fn scala_find_with_err() {
        let path = Path::new("tests/resources/scala/without-comments.scala");
        let result = Scala {
            maybe_file: File::open(path),
            file_name: str(IRRELEVANT),
            multi_opts: scala::multi_opts(),
        }
        .find();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().lines.len(), 0);
    }

    #[test]
    fn css_find_with_value() {
        let path = Path::new("tests/resources/css/with-comments.css");
        let result = CSS {
            maybe_file: File::open(path),
            file_name: str(IRRELEVANT),
            multi_opts: css::multi_opts(),
        }
        .find();
        assert!(result.is_ok());
        let lines = result.unwrap().lines;
        assert_eq!(lines.len(), 6);
        assert_eq!(lines, [1, 3, 6, 7, 8, 10]);
    }

    #[test]
    fn css_find_with_err() {
        let path = Path::new("tests/resources/css/without-comments.css");
        let result = CSS {
            maybe_file: File::open(path),
            file_name: str(IRRELEVANT),
            multi_opts: css::multi_opts(),
        }
        .find();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().lines.len(), 0);
    }

    #[test]
    fn html_find_with_value() {
        let path = Path::new("tests/resources/html/with-comments.html");
        let result = HTML {
            maybe_file: File::open(path),
            file_name: str(IRRELEVANT),
            multi_opts: html::multi_opts(),
        }
        .find();
        assert!(result.is_ok());
        let lines = result.unwrap().lines;
        assert_eq!(lines.len(), 6);
        assert_eq!(lines, [1, 3, 5, 7, 8, 9]);
    }

    #[test]
    fn html_find_with_err() {
        let path = Path::new("tests/resources/html/without-comments.html");
        let result = HTML {
            maybe_file: File::open(path),
            file_name: str(IRRELEVANT),
            multi_opts: html::multi_opts(),
        }
        .find();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().lines.len(), 0);
    }

    #[test]
    fn c_find_with_value() {
        let path = Path::new("tests/resources/c/with-comments.c");
        let result = C {
            maybe_file: File::open(path),
            file_name: str(IRRELEVANT),
            multi_opts: c::multi_opts(),
        }
        .find();
        assert!(result.is_ok());
        let lines = result.unwrap().lines;
        assert_eq!(lines.len(), 8);
        assert_eq!(lines, [1, 3, 5, 6, 7, 9, 10, 12]);
    }

    #[test]
    fn c_find_with_err() {
        let path = Path::new("tests/resources/c/without-comments.c");
        let result = C {
            maybe_file: File::open(path),
            file_name: str(IRRELEVANT),
            multi_opts: c::multi_opts(),
        }
        .find();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().lines.len(), 0);
    }

    #[test]
    fn cpp_find_with_value() {
        let path = Path::new("tests/resources/cpp/with-comments.cpp");
        let result = Cpp {
            maybe_file: File::open(path),
            file_name: str(IRRELEVANT),
            multi_opts: cpp::multi_opts(),
        }
        .find();
        assert!(result.is_ok());
        let lines = result.unwrap().lines;
        assert_eq!(lines.len(), 11);
        assert_eq!(lines, [1, 3, 5, 6, 7, 9, 10, 12, 14, 15, 16]);
    }

    #[test]
    fn cpp_find_with_err() {
        let path = Path::new("tests/resources/cpp/without-comments.cpp");
        let result = Cpp {
            maybe_file: File::open(path),
            file_name: str(IRRELEVANT),
            multi_opts: cpp::multi_opts(),
        }
        .find();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().lines.len(), 0);
    }

    #[test]
    fn lua_find_with_value() {
        let path = Path::new("tests/resources/lua/with-comments.lua");
        let result = Lua {
            maybe_file: File::open(path),
            file_name: str(IRRELEVANT),
        }
        .find();
        assert!(result.is_ok());
        let lines = result.unwrap().lines;
        assert_eq!(lines.len(), 13);
        assert_eq!(lines, [1, 2, 5, 6, 7, 9, 10, 11, 13, 14, 15, 19, 21]);
    }

    #[test]
    fn lua_find_with_err() {
        let path = Path::new("tests/resources/lua/without-comments.lua");
        let result = Lua {
            maybe_file: File::open(path),
            file_name: str(IRRELEVANT),
        }
        .find();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().lines.len(), 0);
    }
}

#[cfg(test)]
mod flags {
    use commentective::utils::string::str;
    use commentective::OptionsCli;
    use std::path::Path;

    #[test]
    fn flag_extension() {
        let path_javascript = Path::new("tests/resources/javascript/with-comments.js");
        let path_python = Path::new("tests/resources/python/with-comments.py");
        let paths = vec![path_javascript, path_python];

        let opts_no_ext = OptionsCli {
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

        let opts_with_ext = OptionsCli {
            extension: Some(str("js")),
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
    use commentective::OptionsCli;
    use std::path::Path;

    const EXISTING_FILE: &'static str = "tests/resources/javascript/with-comments.js";
    const UNSUPPORTED_FILE: &'static str = "tests/resources/empty.foo";

    #[test]
    fn resolve_type_with_value() {
        let path = Path::new(EXISTING_FILE);
        let result = commentective::resolve_type_and_run(
            path,
            &OptionsCli {
                extension: None,
                short: false,
                ignore_empty: false,
            },
        );
        assert!(result.is_ok());
    }

    #[test]
    fn resolve_type_with_err() {
        let path = Path::new(UNSUPPORTED_FILE);
        let result = commentective::resolve_type_and_run(
            path,
            &OptionsCli {
                extension: None,
                short: false,
                ignore_empty: false,
            },
        );
        assert!(result.is_err());
    }
}

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
    use commentective::language::Finder;
    use std::path::PathBuf;

    #[test]
    fn javascript__find__with_comments() {
        let path = PathBuf::from("tests/resources/javascript/with-comments.js");
        let result = JavaScript::with_finder(Finder {}).find(path);
        assert!(result.is_ok());
        let lines = result.unwrap().lines;
        assert_eq!(lines.len(), 13);
        assert_eq!(lines, [1, 3, 4, 5, 7, 8, 9, 11, 12, 13, 15, 17, 18]);
    }

    #[test]
    fn javascript__find__without_comments() {
        let path = PathBuf::from("tests/resources/javascript/without-comments.js");
        let result = JavaScript::with_finder(Finder {}).find(path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().lines.len(), 0);
    }

    #[test]
    fn java__find__with_comments() {
        let path = PathBuf::from("tests/resources/java/with-comments.java");
        let result = Java::with_finder(Finder {}).find(path);
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
        let result = Java::with_finder(Finder {}).find(path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().lines.len(), 0);
    }

    #[test]
    fn rust__find__with_comments() {
        let path = PathBuf::from("tests/resources/rust/with-comments.rs");
        let result = Rust::with_finder(Finder {}).find(path);
        assert!(result.is_ok());
        let lines = result.unwrap().lines;
        assert_eq!(lines.len(), 10);
        assert_eq!(lines, [1, 4, 5, 6, 9, 10, 11, 13, 15, 16]);
    }

    #[test]
    fn rust__find__without_comments() {
        let path = PathBuf::from("tests/resources/rust/without-comments.rs");
        let result = Rust::with_finder(Finder {}).find(path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().lines.len(), 0);
    }

    #[test]
    fn python__find__with_comments() {
        let path = PathBuf::from("tests/resources/python/with-comments.py");
        let result = Python::with_finder(Finder {}).find(path);
        assert!(result.is_ok());
        let lines = result.unwrap().lines;
        assert_eq!(lines.len(), 4);
        assert_eq!(lines, [1, 3, 4, 5]);
    }

    #[test]
    fn python__find__without_comments() {
        let path = PathBuf::from("tests/resources/python/without-comments.py");
        let result = Python::with_finder(Finder {}).find(path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().lines.len(), 0);
    }

    #[test]
    fn csharp__find__with_comments() {
        let path = PathBuf::from("tests/resources/csharp/with-comments.cs");
        let result = CSharp::with_finder(Finder {}).find(path);
        assert!(result.is_ok());
        let lines = result.unwrap().lines;
        assert_eq!(lines.len(), 9);
        assert_eq!(lines, [1, 3, 5, 6, 7, 8, 10, 12, 13])
    }

    #[test]
    fn csharp__find__without_comments() {
        let path = PathBuf::from("tests/resources/csharp/without-comments.cs");
        let result = CSharp::with_finder(Finder {}).find(path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().lines.len(), 0);
    }

    #[test]
    fn bash__find__with_comments() {
        let path = PathBuf::from("tests/resources/bash/with-comments.sh");
        let result = Bash::with_finder(Finder {}).find(path);
        assert!(result.is_ok());
        let lines = result.unwrap().lines;
        assert_eq!(lines.len(), 5);
        assert_eq!(lines, [3, 6, 7, 8, 9]);
    }

    #[test]
    fn bash__find__without_comments() {
        let path = PathBuf::from("tests/resources/bash/without-comments.sh");
        let result = Bash::with_finder(Finder {}).find(path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().lines.len(), 0);
    }

    #[test]
    fn php__find__with_comments() {
        let path = PathBuf::from("tests/resources/php/with-comments.php");
        let result = PHP::with_finder(Finder {}).find(path);
        assert!(result.is_ok());
        let lines = result.unwrap().lines;
        assert_eq!(lines.len(), 11);
        assert_eq!(lines, [3, 5, 7, 9, 10, 11, 13, 14, 15, 17, 18]);
    }

    #[test]
    fn php__find__without_comments() {
        let path = PathBuf::from("tests/resources/php/without-comments.php");
        let result = PHP::with_finder(Finder {}).find(path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().lines.len(), 0);
    }

    #[test]
    fn ruby__find__with_comments() {
        let path = PathBuf::from("tests/resources/ruby/with-comments.rb");
        let result = Ruby::with_finder(Finder {}).find(path);
        assert!(result.is_ok());
        let lines = result.unwrap().lines;
        assert_eq!(lines.len(), 6);
        assert_eq!(lines, [1, 2, 3, 5, 8, 9]);
    }

    #[test]
    fn ruby__find__without_comments() {
        let path = PathBuf::from("tests/resources/ruby/without-comments.rb");
        let result = Ruby::with_finder(Finder {}).find(path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().lines.len(), 0);
    }

    #[test]
    fn golang__find__with_comments() {
        let path = PathBuf::from("tests/resources/golang/with-comments.go");
        let result = Go::with_finder(Finder {}).find(path);
        assert!(result.is_ok());
        let lines = result.unwrap().lines;
        assert_eq!(lines.len(), 2);
        assert_eq!(lines, [1, 2]);
    }

    #[test]
    fn golang__find__without_comments() {
        let path = PathBuf::from("tests/resources/golang/without-comments.go");
        let result = Go::with_finder(Finder {}).find(path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().lines.len(), 0);
    }

    #[test]
    fn scala__find__with_comments() {
        let path = PathBuf::from("tests/resources/scala/with-comments.scala");
        let result = Scala::with_finder(Finder {}).find(path);
        assert!(result.is_ok());
        let lines = result.unwrap().lines;
        assert_eq!(lines.len(), 11);
        assert_eq!(lines, [1, 7, 8, 9, 11, 12, 13, 15, 16, 18, 19]);
    }

    #[test]
    fn scala__find__without_comments() {
        let path = PathBuf::from("tests/resources/scala/without-comments.scala");
        let result = Scala::with_finder(Finder {}).find(path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().lines.len(), 0);
    }

    #[test]
    fn css__find__with_comments() {
        let path = PathBuf::from("tests/resources/css/with-comments.css");
        let result = CSS::with_finder(Finder {}).find(path);
        assert!(result.is_ok());
        let lines = result.unwrap().lines;
        assert_eq!(lines.len(), 6);
        assert_eq!(lines, [1, 3, 6, 7, 8, 10]);
    }

    #[test]
    fn css__find__without_comments() {
        let path = PathBuf::from("tests/resources/css/without-comments.css");
        let result = CSS::with_finder(Finder {}).find(path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().lines.len(), 0);
    }

    #[test]
    fn html__find__with_comments() {
        let path = PathBuf::from("tests/resources/html/with-comments.html");
        let result = HTML::with_finder(Finder {}).find(path);
        assert!(result.is_ok());
        let lines = result.unwrap().lines;
        assert_eq!(lines.len(), 6);
        assert_eq!(lines, [1, 3, 5, 7, 8, 9]);
    }

    #[test]
    fn html__find__without_comments() {
        let path = PathBuf::from("tests/resources/html/without-comments.html");
        let result = HTML::with_finder(Finder {}).find(path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().lines.len(), 0);
    }

    #[test]
    fn c__find__with_comments() {
        let path = PathBuf::from("tests/resources/c/with-comments.c");
        let result = C::with_finder(Finder {}).find(path);
        assert!(result.is_ok());
        let lines = result.unwrap().lines;
        assert_eq!(lines.len(), 8);
        assert_eq!(lines, [1, 3, 5, 6, 7, 9, 10, 12]);
    }

    #[test]
    fn c__find__without_comments() {
        let path = PathBuf::from("tests/resources/c/without-comments.c");
        let result = C::with_finder(Finder {}).find(path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().lines.len(), 0);
    }

    #[test]
    fn cpp__find__with_comments() {
        let path = PathBuf::from("tests/resources/cpp/with-comments.cpp");
        let result = Cpp::with_finder(Finder {}).find(path);
        assert!(result.is_ok());
        let lines = result.unwrap().lines;
        assert_eq!(lines.len(), 11);
        assert_eq!(lines, [1, 3, 5, 6, 7, 9, 10, 12, 14, 15, 16]);
    }

    #[test]
    fn cpp__find__without_comments() {
        let path = PathBuf::from("tests/resources/cpp/without-comments.cpp");
        let result = Cpp::with_finder(Finder {}).find(path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().lines.len(), 0);
    }

    #[test]
    fn lua__find__with_comments() {
        let path = PathBuf::from("tests/resources/lua/with-comments.lua");
        let result = Lua::with_finder(Finder {}).find(path);
        assert!(result.is_ok());
        let lines = result.unwrap().lines;
        assert_eq!(lines.len(), 13);
        assert_eq!(lines, [1, 2, 5, 6, 7, 9, 10, 11, 13, 14, 15, 19, 21]);
    }

    #[test]
    fn lua__find__without_comments() {
        let path = PathBuf::from("tests/resources/lua/without-comments.lua");
        let result = Lua::with_finder(Finder {}).find(path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().lines.len(), 0);
    }
}

extern crate detector;

#[cfg(test)]
mod language_types {
    use detector;
    use detector::language as l;
    use detector::language::Language;
    use std::fs::File;
    use std::path::Path;

    #[test]
    fn js_find_with_comments() {
        let path = Path::new("tests/resources/js-with-comments.js");
        let result = l::javascript::Struct {
            file: File::open(path),
        }.find();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().lines.len(), 8);
    }

    #[test]
    fn js_find_without_comments() {
        let path = Path::new("tests/resources/js-without-comments.js");
        let result = l::javascript::Struct {
            file: File::open(path),
        }.find();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().lines.len(), 0);
    }

    #[test]
    fn bin_run_with_unsupported_file() {
        let path = Path::new("tests/resources/unsupported.arb");
        let result = detector::run(path);
        assert!(result.is_err());
    }
}

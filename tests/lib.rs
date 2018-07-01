extern crate detector;

#[cfg(test)]
mod language_types {
    use detector::language;
    use detector::language::Language;
    use std::fs::File;
    use std::path::Path;

    #[test]
    fn find_javascript_comments() {
        let file = File::open(Path::new("tests/resources/example-js.js"));
        let result = language::javascript::JavaScript::find(file.unwrap());
        assert_eq!(result.unwrap().lines.len(), 8);
    }
}

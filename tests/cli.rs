#[cfg(test)]
mod cli {
    #![allow(non_snake_case)]

    use assert_cmd::prelude::*;
    use predicates::str::{similar, starts_with};
    use std::io::Write;
    use std::process::Command;

    type TestResult = Result<(), Box<dyn std::error::Error>>;

    #[test]
    fn test_output__no_args__shows_error() -> TestResult {
        let mut cmd = Command::cargo_bin("commentective")?;

        let expected = "error: The following required arguments were not provided:
    <FILES>...";

        cmd.assert().failure().stderr(starts_with(expected));
        Ok(())
    }

    #[test]
    fn test_output__finds_comments() -> TestResult {
        let mut file = tempfile::Builder::new()
            .prefix("arbitrary")
            .suffix(".js")
            .tempfile()?;
        file.write_all(
            "const someVar = true;\n// comment\n/* multi comment\nanother line\nend line*/"
                .as_bytes(),
        )?;

        let mut cmd = Command::cargo_bin("commentective")?;
        cmd.arg(file.path());

        let path_buf = file.path().to_path_buf();
        let file_name = path_buf.file_name().unwrap().to_str().unwrap();
        let expected = format!(
            "────────────────────────────────────────────────────────────────────────────────\nFile: {}\n────────────────────────────────────────────────────────────────────────────────\nL2\nL3\nL4\nL5\n", file_name);

        cmd.assert().success().stdout(similar(expected));
        Ok(())
    }

    #[test]
    fn test_output__only_processes_specified_extension_type() -> TestResult {
        let mut js_file = tempfile::Builder::new()
            .prefix("arbitrary")
            .suffix(".js")
            .tempfile()?;
        js_file.write_all(
            "const someVar = true;\n// comment\n/* multi comment\nanother line\nend line*/"
                .as_bytes(),
        )?;

        let mut rs_file = tempfile::Builder::new()
            .prefix("arbitrary")
            .suffix(".rs")
            .tempfile()?;
        rs_file.write_all(
            "let some_var = true;\n// comment\n/* multi comment\nanother line\nend line*/"
                .as_bytes(),
        )?;

        let mut cmd = Command::cargo_bin("commentective")?;
        cmd.arg(js_file.path());
        cmd.arg(rs_file.path());
        cmd.arg("--extension");
        cmd.arg("rs");

        let path_buf = rs_file.path().to_path_buf();
        let rs_file_name = path_buf.file_name().unwrap().to_str().unwrap().clone();
        let expected = format!(
            "────────────────────────────────────────────────────────────────────────────────\nFile: {}\n────────────────────────────────────────────────────────────────────────────────\nL2\nL3\nL4\nL5\n", rs_file_name);

        cmd.assert().success().stdout(similar(expected));
        Ok(())
    }

    #[test]
    fn test_output__with_short_flag() -> TestResult {
        let mut file = tempfile::Builder::new()
            .prefix("arbitrary")
            .suffix(".js")
            .tempfile()?;
        file.write_all(
            "const someVar = true;\n// comment\n/* multi comment\nanother line\nend line*/"
                .as_bytes(),
        )?;

        let mut cmd = Command::cargo_bin("commentective")?;
        cmd.arg(file.path());
        cmd.arg("--short");

        let path_buf = file.path().to_path_buf();
        let file_name = path_buf.file_name().unwrap().to_str().unwrap().clone();
        let expected = format!(
            "{}:2\n{}:3\n{}:4\n{}:5\n",
            file_name, file_name, file_name, file_name
        );

        cmd.assert().success().stdout(similar(expected));
        Ok(())
    }

    #[test]
    fn test_output__with_ignore_empty_flag() -> TestResult {
        let mut file = tempfile::Builder::new()
            .prefix("arbitrary")
            .suffix(".js")
            .tempfile()?;
        file.write_all(
            "const someVar = true;\n// comment\n/* multi comment\nanother line\nend line*/"
                .as_bytes(),
        )?;

        let empty_file = tempfile::Builder::new()
            .prefix("arbitrary")
            .suffix(".rs")
            .tempfile()?;

        let mut cmd = Command::cargo_bin("commentective")?;
        cmd.arg(file.path());
        cmd.arg(empty_file.path());
        cmd.arg("--ignore-empty");

        let path_buf = file.path().to_path_buf();
        let file_name = path_buf.file_name().unwrap().to_str().unwrap().clone();
        let expected = format!("────────────────────────────────────────────────────────────────────────────────\nFile: {}\n────────────────────────────────────────────────────────────────────────────────\nL2\nL3\nL4\nL5\n", file_name);

        cmd.assert().success().stdout(similar(expected));
        Ok(())
    }

    #[test]
    fn test_output__with_unsupported_file() -> TestResult {
        let unsupported_file = tempfile::Builder::new()
            .prefix("arbitrary")
            .suffix(".unsupported")
            .tempfile()?;

        let mut cmd = Command::cargo_bin("commentective")?;
        cmd.arg(unsupported_file.path());

        let buf = unsupported_file.path().to_path_buf();
        let file_name = buf.to_str().unwrap_or("something went wrong");
        let expected = format!("────────────────────────────────────────────────────────────────────────────────\nError: Unsupported file extension for path: {}\n────────────────────────────────────────────────────────────────────────────────\n", file_name);

        cmd.assert().failure().stdout(similar(expected));
        Ok(())
    }
}

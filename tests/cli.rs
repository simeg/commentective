#[cfg(test)]
mod cli {
    #![allow(non_snake_case)]

    use assert_cmd::prelude::*;
    use predicates::str::{diff, starts_with};
    use std::ffi::OsStr;
    use std::io::Write;
    use std::process::Command;
    use tempfile::NamedTempFile;

    type TestResult = Result<(), Box<dyn std::error::Error>>;

    #[test]
    fn output__no_args__shows_error() -> TestResult {
        let mut cmd = Command::cargo_bin("commentective")?;

        let expected = "error: The following required arguments were not provided:
    <FILES>...";

        cmd.assert().failure().stderr(starts_with(expected));
        Ok(())
    }

    #[test]
    fn output__finds_comments() -> TestResult {
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

        let file_name = get_file_name(&mut file);
        let expected = format!(
            "────────────────────────────────────────────────────────────────────────────────\nFile: {}\n────────────────────────────────────────────────────────────────────────────────\nL2\nL3\nL4\nL5\n", file_name);

        cmd.assert().success().stdout(diff(expected));
        Ok(())
    }

    #[test]
    fn output__with_short_flag() -> TestResult {
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

        let file_name = get_file_name(&mut file);
        let expected = format!(
            "{}:2\n{}:3\n{}:4\n{}:5\n",
            file_name, file_name, file_name, file_name
        );

        cmd.assert().success().stdout(diff(expected));
        Ok(())
    }

    #[test]
    fn output__with_ignore_empty_flag() -> TestResult {
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

        let file_name = get_file_name(&mut file);
        let expected = format!("────────────────────────────────────────────────────────────────────────────────\nFile: {}\n────────────────────────────────────────────────────────────────────────────────\nL2\nL3\nL4\nL5\n", file_name);

        cmd.assert().success().stdout(diff(expected));
        Ok(())
    }

    #[test]
    fn output__with_unsupported_file() -> TestResult {
        let unsupported_file = tempfile::Builder::new()
            .prefix("arbitrary")
            .suffix(".unsupported")
            .tempfile()?;

        let mut cmd = Command::cargo_bin("commentective")?;
        cmd.arg(unsupported_file.path());

        let buf = unsupported_file.path().to_path_buf();
        let full_path = buf.to_str().unwrap_or("something went wrong");
        let expected = format!("────────────────────────────────────────────────────────────────────────────────\nError: Unsupported file extension for path: {}\n────────────────────────────────────────────────────────────────────────────────\n", full_path);

        cmd.assert().failure().stdout(diff(expected));
        Ok(())
    }

    #[test]
    fn output__with_code_flag() -> TestResult {
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
        cmd.arg("--code");

        let file_name = get_file_name(&mut file);
        let expected = format!("────────────────────────────────────────────────────────────────────────────────\nFile: {}\n────────────────────────────────────────────────────────────────────────────────\nL2 - // comment\nL3 - /* multi comment\nL4 - another line\nL5 - end line*/\n", file_name);

        cmd.assert().success().stdout(diff(expected));
        Ok(())
    }

    #[test]
    fn output__with_lang_option() -> TestResult {
        let mut file = tempfile::Builder::new()
            .prefix("arbitrary")
            .suffix(".some_other_extension")
            .tempfile()?;
        file.write_all(
            "const someVar = true;\n// comment\n/* multi comment\nanother line\nend line*/"
                .as_bytes(),
        )?;

        let mut cmd = Command::cargo_bin("commentective")?;
        cmd.arg(file.path());
        cmd.arg("--lang");
        cmd.arg("js");

        let file_name = get_file_name(&mut file);
        let expected = format!("────────────────────────────────────────────────────────────────────────────────\nFile: {}\n────────────────────────────────────────────────────────────────────────────────\nL2\nL3\nL4\nL5\n", file_name);

        cmd.assert().success().stdout(diff(expected));
        Ok(())
    }

    fn get_file_name(file: &mut NamedTempFile) -> String {
        let path_buf = file.path().to_path_buf();
        path_buf
            .file_name()
            .map(OsStr::to_str)
            .flatten()
            .unwrap_or("something went wrong")
            .to_string()
    }
}

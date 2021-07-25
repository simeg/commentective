extern crate colored;

use self::colored::ColoredString;
use self::colored::*;
use console::Term;

use crate::language::Comment;
use crate::printer::Color::{Green, Yellow};

use std::io;
use std::io::{Error, Write};

enum Color {
    Green,
    Red,
    White,
    Yellow,
}

pub struct Printer<W> {
    writer: W,
}

impl<W> Printer<W>
where
    W: Write,
{
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    pub fn print_file_name(&mut self, file_name: &str) -> io::Result<()> {
        self.print_colored_line(Yellow)?;
        self.print_colored_text(format!("{} {}", "File:", file_name), Yellow)?;
        self.print_colored_line(Yellow)
    }

    pub fn print_comments(
        &mut self,
        comment: Comment,
        file_name: &str,
        is_short: bool,
        is_code: bool,
    ) -> io::Result<()> {
        if is_short {
            let msg = if is_code {
                format!("{}:{}:{}", file_name, comment.line_number, comment.content)
            } else {
                format!("{}:{}", file_name, comment.line_number)
            };
            writeln!(&mut self.writer, "{}", msg)
        } else {
            let msg = if is_code {
                format!("L{} - {}", comment.line_number, comment.content)
            } else {
                format!("L{}", comment.line_number)
            };
            self.print_colored_text(msg, Green)
        }
    }

    pub fn print_no_comments_found(&mut self, file_name: String) -> Result<(), Error> {
        self.print_file_name(file_name.as_str())?;
        self.print_colored_text(String::from("> No comments found"), Color::White)
    }

    pub fn print_error(&mut self, err: &Error) -> io::Result<()> {
        self.print_colored_line(Color::Red)?;
        self.print_colored_text(format!("Error: {}", err.to_string()), Color::Red)?;
        self.print_colored_line(Color::Red)
    }

    fn print_colored_line(&mut self, color: Color) -> io::Result<()> {
        let term_width = Term::stdout().size().1 as usize;
        let line = "─".repeat(term_width);
        self.print_colored_text(line, color)
    }

    fn print_colored_text(&mut self, text: String, color: Color) -> io::Result<()> {
        match color {
            Color::Yellow => self.println(text.yellow()),
            Color::Green => self.println(text.green()),
            Color::Red => self.println(text.red()),
            Color::White => self.println(text.white()),
        }
    }

    fn println(&mut self, output: ColoredString) -> io::Result<()> {
        writeln!(&mut self.writer, "{}", output)
    }
}

#[cfg(test)]
mod test {
    #![allow(non_snake_case)]

    use super::colored::Colorize;
    use crate::language::Comment;
    use crate::printer::{Color, Printer};
    use console::Term;
    use predicates::prelude::predicate::str::contains;
    use predicates::str::{ends_with, starts_with};
    use predicates::Predicate;
    use std::io::{Error, ErrorKind};

    #[test]
    fn test_printer__print_file_name() {
        let mut output = Vec::new();
        let mut printer = Printer::new(&mut output);
        let term_width = Term::stdout().size().1 as usize;

        printer.print_file_name("some-file-name").unwrap();

        let actual = String::from_utf8(output).expect("Not UTF-8");

        assert!(starts_with(format!("{}\n", "─".repeat(term_width))).eval(&actual));
        assert!(contains("──\nFile: some-file-name\n──").eval(&actual));
        assert!(ends_with(format!("{}\n", "─".repeat(term_width))).eval(&actual));
    }

    #[test]
    fn test_printer__print_comments__short_and_bold() {
        let mut output = Vec::new();
        let mut printer = Printer::new(&mut output);

        let is_short = true;
        let is_code = true;
        printer
            .print_comments(
                Comment::new(1, "some-content".to_string()),
                "some-file-name",
                is_short,
                is_code,
            )
            .unwrap();

        let actual = String::from_utf8(output).expect("Not UTF-8");
        let expected = "some-file-name:1:some-content\n";

        assert_eq!(actual, expected)
    }

    #[test]
    fn test_printer__print_comments__short_but_not_bold() {
        let mut output = Vec::new();
        let mut printer = Printer::new(&mut output);

        let is_short = true;
        let is_code = false;
        printer
            .print_comments(
                Comment::new(1, "some-content".to_string()),
                "some-file-name",
                is_short,
                is_code,
            )
            .unwrap();

        let actual = String::from_utf8(output).expect("Not UTF-8");
        let expected = "some-file-name:1\n";

        assert_eq!(actual, expected)
    }

    #[test]
    fn test_printer__print_comments__not_short_but_bold() {
        let mut output = Vec::new();
        let mut printer = Printer::new(&mut output);

        let is_short = false;
        let is_code = true;
        printer
            .print_comments(
                Comment::new(1, "some-content".to_string()),
                "some-file-name",
                is_short,
                is_code,
            )
            .unwrap();

        let actual = String::from_utf8(output).expect("Not UTF-8");
        let expected = "L1 - some-content\n";

        assert_eq!(actual, expected)
    }

    #[test]
    fn test_printer__print_comments__not_short_or_bold() {
        let mut output = Vec::new();
        let mut printer = Printer::new(&mut output);

        let is_short = false;
        let is_code = false;
        printer
            .print_comments(
                Comment::new(1, "some-content".to_string()),
                "some-file-name",
                is_short,
                is_code,
            )
            .unwrap();

        let actual = String::from_utf8(output).expect("Not UTF-8");
        let expected = "L1\n";

        assert_eq!(actual, expected)
    }

    #[test]
    fn test_printer__print_no_comments_found() {
        let mut output = Vec::new();
        let mut printer = Printer::new(&mut output);
        let term_width = Term::stdout().size().1 as usize;

        printer
            .print_no_comments_found("some-file-name".to_string())
            .unwrap();

        let actual = String::from_utf8(output).expect("Not UTF-8");

        assert!(starts_with(format!("{}\n", "─".repeat(term_width))).eval(&actual));
        assert!(contains("──\nFile: some-file-name\n──").eval(&actual));
        assert!(
            ends_with(format!("{}\n> No comments found\n", "─".repeat(term_width))).eval(&actual)
        );
    }

    #[test]
    fn test_printer__print_error() {
        let mut output = Vec::new();
        let mut printer = Printer::new(&mut output);
        let term_width = Term::stdout().size().1 as usize;

        printer
            .print_error(&Error::new(ErrorKind::NotFound, "some-error-msg"))
            .unwrap();

        let actual = String::from_utf8(output).expect("Not UTF-8");

        assert!(starts_with(format!("{}\n", "─".repeat(term_width))).eval(&actual));
        assert!(contains("──\nError: some-error-msg\n──").eval(&actual));
        assert!(ends_with(format!("{}\n", "─".repeat(term_width))).eval(&actual));
    }

    #[test]
    fn test_printer__print_colored_line() {
        let mut output = Vec::new();
        let mut printer = Printer::new(&mut output);
        let term_width = Term::stdout().size().1 as usize;

        // Color is irrelevant as we're ignoring colors in these tests
        printer.print_colored_line(Color::Yellow).unwrap();

        let actual = String::from_utf8(output).expect("Not UTF-8");
        let expected = format!("{}\n", "─".repeat(term_width));

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_printer__print_colored_text() {
        let mut output = Vec::new();
        let mut printer = Printer::new(&mut output);

        // Color is irrelevant as we're ignoring colors in these tests
        printer
            .print_colored_text("some-text".to_string(), Color::Yellow)
            .unwrap();

        let actual = String::from_utf8(output).expect("Not UTF-8");
        let expected = "some-text\n";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_printer__println() {
        let mut output = Vec::new();
        let mut printer = Printer::new(&mut output);

        // Color is irrelevant as we're ignoring colors in these tests
        printer.println("some-text".yellow()).unwrap();

        let actual = String::from_utf8(output).expect("Not UTF-8");
        let expected = "some-text\n";

        assert_eq!(actual, expected);
    }
}

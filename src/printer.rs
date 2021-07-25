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
    pub writer: W,
}

impl<W> Printer<W>
where
    W: Write,
{
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

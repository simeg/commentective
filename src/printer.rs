extern crate colored;

use self::colored::ColoredString;
use self::colored::*;
use crate::language::FindResult;
use crate::printer::Color::{Green, Red, White, Yellow};
use crate::CommentativeOpts;
use console::Term;
use std::io::Error;
use std::io::Write;

enum Color {
    Green,
    Red,
    White,
    Yellow,
}

pub struct Printer<'a, W> {
    pub writer: W,
    pub options: &'a CommentativeOpts,
}

impl<'a, W> Printer<'a, W>
where
    W: Write,
{
    pub fn terminal(&mut self, maybe_result: Result<FindResult, Error>) -> Result<(), Error> {
        match maybe_result {
            Ok(result) => {
                if !result.print {
                    return Ok(());
                }
                match result.lines.len() {
                    0 => {
                        if self.options.short || self.options.ignore_empty {
                            // Don't print if nothing found
                        } else {
                            self.print_file_name(&result.file_name);
                            self.print_colored(String::from("> No comments found"), White);
                        }
                        Ok(())
                    }
                    _ => {
                        self.result(result);
                        Ok(())
                    }
                }
            }
            Err(e) => {
                self.error(&e);
                Err(e)
            }
        }
    }

    fn error(&mut self, err: &Error) {
        if !self.options.short {
            self.print_colored_line(Red);
        }
        let msg = format!("Error: {}", err.to_string());
        if !self.options.short {
            self.print_colored(msg, Red);
            self.print_colored_line(Red);
        }
    }

    fn result(&mut self, result: FindResult) {
        let file_name = result.file_name;
        if !self.options.short {
            self.print_file_name(&file_name);
        }
        result
            .lines
            .into_iter()
            .map(|line| self.print_comments(line, &file_name))
            .for_each(drop);
    }

    fn print_file_name(&mut self, file_name: &str) {
        self.print_colored_line(Yellow);
        let msg = format!("{} {}", "File:", file_name);
        self.print_colored(msg, Yellow);
        self.print_colored_line(Yellow);
    }

    fn print_comments(&mut self, line: (u32, String), file_name: &str) {
        if self.options.short {
            let msg = if self.options.code {
                format!("{}:{}:{}", file_name, line.0, line.1)
            } else {
                format!("{}:{}", file_name, line.0)
            };
            writeln!(&mut self.writer, "{}", msg).expect("Unable to write")
        } else {
            let msg = if self.options.code {
                format!("L{} - {}", line.0, line.1)
            } else {
                format!("L{}", line.0)
            };
            self.print_colored(msg, Green);
        }
    }

    fn print_colored_line(&mut self, color: Color) {
        let term_width = Term::stdout().size().1 as usize;
        let line = "─".repeat(term_width);
        self.print_colored(line, color);
    }

    fn print_colored(&mut self, text: String, color: Color) {
        match color {
            Color::Yellow => self.writeln(text.yellow()),
            Color::Green => self.writeln(text.green()),
            Color::Red => self.writeln(text.red()),
            Color::White => self.writeln(text.white()),
        }
    }

    fn writeln(&mut self, output: ColoredString) {
        writeln!(&mut self.writer, "{}", output).expect("Unable to write")
    }
}

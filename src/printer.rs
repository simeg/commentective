extern crate colored;

use self::colored::ColoredString;
use self::colored::*;
use console::Term;
use language::FindResult;
use printer::Colors::*;
use std::io::Error;
use std::io::Write;

enum Colors {
    Green,
    Red,
    White,
    Yellow,
}

pub struct Printer<W> {
    pub writer: W,
    pub short: bool
}

impl<W> Printer<W>
where
    W: Write,
{
    pub fn terminal(&mut self, maybe_result: Result<FindResult, Error>) -> Result<(), Error> {
        match maybe_result {
            Ok(result) => match result.lines.len() {
                0 => {
                    if !self.short {
                        self.print_file_name(&result.file_name);
                        self.print_colored(String::from("> No comments found"), White);
                    }
                    Ok(())
                }
                _ => {
                    self.result(result);
                    Ok(())
                }
            },
            Err(e) => {
                self.error(&e);
                Err(e)
            }
        }
    }

    fn error(&mut self, err: &Error) {
        if !self.short {
            self.print_colored_line(Red);
        }
        let msg = format!("Error: {}", err.to_string());
        if !self.short {
            self.print_colored(msg, Red);
            self.print_colored_line(Red);
        }
    }

    fn result(&mut self, result: FindResult) {
        let file_name = result.file_name;
        if !self.short {
            self.print_file_name(&file_name);
        }
        result
            .lines
            .into_iter()
            .map(|line| self.print_comments(line, &file_name))
            .for_each(drop);
    }

    fn print_file_name(&mut self, file_name: &String) {
        self.print_colored_line(Yellow);
        let msg = format!("{} {}", "File:", file_name);
        self.print_colored(msg, Yellow);
        self.print_colored_line(Yellow);
    }

    fn print_comments(&mut self, line_number: u32, file_name: &String) {
        if !self.short {
            let msg = format!("L{}", line_number.to_string());
            self.print_colored(msg, Green);
        } else {
            let msg = format!("{}:{}", file_name, line_number.to_string());
            writeln!(&mut self.writer, "{}", msg).expect("Unable to write")
        }
    }

    fn print_colored_line(&mut self, color: Colors) {
        let term_width = Term::stdout().size().1 as usize;
        let line = "─".repeat(term_width);
        self.print_colored(line, color);
    }

    fn print_colored(&mut self, text: String, color: Colors) {
        match color {
            Colors::Yellow => self.writeln(text.yellow()),
            Colors::Green => self.writeln(text.green()),
            Colors::Red => self.writeln(text.red()),
            Colors::White => self.writeln(text.white()),
        }
    }

    fn writeln(&mut self, output: ColoredString) {
        writeln!(&mut self.writer, "{}", output).expect("Unable to write")
    }
}

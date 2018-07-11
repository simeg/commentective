extern crate colored;

use self::colored::ColoredString;
use self::colored::*;
use console::Term;
use language::FindResult;
use printer::Colors::Red;
use printer::Colors::Yellow;
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
}

impl<W> Printer<W>
where
    W: Write,
{
    pub fn terminal(&mut self, maybe_result: Result<FindResult, Error>) -> Result<(), Error> {
        match maybe_result {
            Ok(result) => match result.lines.len() {
                0 => {
                    Printer::print_file_name(self, result.file_name);
                    Printer::writeln(self, "> No comments found".white());
                    Ok(())
                }
                _ => {
                    Printer::print_result(self, result);
                    Ok(())
                }
            },
            Err(e) => {
                Printer::print_error(self, &e);
                Err(e)
            }
        }
    }

    fn print_error(&mut self, err: &Error) {
        Printer::print_line(self, Red);
        let msg = format!("Error: {}", err.to_string());
        Printer::writeln(self, msg.red());
        Printer::print_line(self, Red);
    }

    fn print_result(&mut self, result: FindResult) {
        Printer::print_file_name(self, result.file_name);
        result
            .lines
            .into_iter()
            .map(|line| Printer::print_comments(self, line))
            .for_each(drop);
    }

    fn print_file_name(&mut self, file_name: String) {
        Printer::print_line(self, Yellow);
        let msg = format!("{} {}", "File:", file_name);
        Printer::writeln(self, msg.yellow());
        Printer::print_line(self, Yellow);
    }

    fn print_comments(&mut self, line_number: u32) {
        let msg = format!("L{}", line_number.to_string());
        Printer::writeln(self, msg.green());
    }

    fn print_line(&mut self, color: Colors) {
        let term_width = Term::stdout().size().1 as usize;
        let line = "─".repeat(term_width);
        match color {
            Colors::Yellow => Printer::writeln(self, line.yellow()),
            Colors::Green => Printer::writeln(self, line.green()),
            Colors::Red => Printer::writeln(self, line.red()),
            Colors::White => Printer::writeln(self, line.white()),
        }
    }

    fn writeln(&mut self, output: ColoredString) {
        writeln!(&mut self.writer, "{}", output).expect("Unable to write")
    }
}

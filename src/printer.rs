extern crate colored;

use self::colored::ColoredString;
use self::colored::*;
use console::Term;
use language::FindResult;
use printer::Color::White;
use printer::Color::Yellow;
use std::io::Error;
use std::io::Write;

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
    pub fn terminal(&mut self, maybe_result: Result<FindResult, Error>) -> Result<(), Error> {
        match maybe_result {
            Ok(result) => match result.lines.len() {
                0 => {
                    Printer::writeln(self, "No comments found".yellow());
                    Ok(())
                }
                _ => {
                    Printer::print_result(self, result);
                    Ok(())
                }
            },
            Err(e) => {
                let msg = format!("Error: {}", e.to_string());
                eprintln!("{}", msg.red().bold());
                Err(e)
            }
        }
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
        Printer::print_line(self, White);
        Printer::writeln(self, file_name.white().bold());
        Printer::print_line(self, White);
    }

    fn print_comments(&mut self, line_number: u32) {
        let msg = format!("L{}", line_number.to_string());
        Printer::writeln(self, msg.green());
    }

    fn print_line(&mut self, color: Color) {
        let term_width = Term::stdout().size().1 as usize;
        let line = "─".repeat(term_width);
        match color {
            Color::Yellow => Printer::writeln(self, line.yellow()),
            Color::Green => Printer::writeln(self, line.green()),
            Color::Red => Printer::writeln(self, line.red()),
            Color::White => Printer::writeln(self, line.white()),
        }
    }

    fn writeln(&mut self, output: ColoredString) {
        writeln!(&mut self.writer, "{}", output).expect("Unable to write")
    }
}

extern crate colored;

use self::colored::*;
use language::FindResult;
use std::io::Error;
use std::io::Write;

pub struct Printer<W> {
    pub writer: W,
}

impl<W> Printer<W>
where
    W: Write,
{
    pub fn terminal(&mut self, maybe_result: Result<FindResult, Error>) {
        match maybe_result {
            Ok(result) => match result.lines.len() {
                0 => Printer::writeln(self, "No comments found".yellow()),
                _ => Printer::print_result(self, result),
            },
            Err(e) => {
                let msg = format!("Error: {}", e.to_string());
                eprintln!("{}", msg.red().bold());
                // TODO: Status code 1
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
        Printer::writeln(self, file_name.white().bold());
    }

    fn print_comments(&mut self, line_number: u32) {
        let msg = format!("L{}", line_number.to_string());
        Printer::writeln(self, msg.green());
    }

    fn writeln(&mut self, output: ColoredString) {
        writeln!(&mut self.writer, "{}", output).expect("Unable to write")
    }
}

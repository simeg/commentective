extern crate docopt;
#[macro_use]
extern crate serde_derive;
extern crate detector;

use docopt::Docopt;

//const VERSION: &'static str = "1.0";

#[cfg_attr(rustfmt, rustfmt_skip)]
const DOCOPT: &'static str = concat!("
detector

Usage:
  detector

");

#[derive(Debug, Deserialize)]
pub struct Args {
    //    pub arg_file: Vec<String>,
}

fn main() {
    let args: Args = Docopt::new(DOCOPT)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    detector::run_lib();
}

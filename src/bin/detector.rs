extern crate docopt;
#[macro_use]
extern crate serde_derive;
extern crate detector;

use docopt::Docopt;
use std::path::Path;

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
    let _args: Args = Docopt::new(DOCOPT)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let path = Path::new("example-js.js");
    let _ = detector::run(path);
}

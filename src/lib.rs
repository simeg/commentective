use language::Comments;
use language::Language;
use std::ffi::OsStr;
use std::fs::File;
use std::io::BufReader;
use std::io::Error;
use std::path::Path;

pub mod language;

pub fn run_lib() {
    let path = Path::new("example-js.js");
    let file = File::open(path);

    let result: Result<Comments, Error> = match path.extension() {
        None => panic!("Could not resolve file extension"),
        Some(_extension) => language::javascript::JavaScript::find(file.unwrap()),
    };

    match result {
        Ok(res) => {
            println!("{:?}", res.lines);
        }
        Err(_) => unreachable!(),
    }
}

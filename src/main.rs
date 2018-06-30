use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let filename = "example-js.js";
    let mut counter = 1; // Lines begin on index 1
    let mut comments = Vec::new();
    let mut flag = false;

    let f = File::open(filename).expect("file not found");
    for line in BufReader::new(f).lines() {
        match line {
            Ok(l) => {
                if is_comment(l, flag) {
                    comments.push(counter);
                }
            }
            Err(_) => panic!("Could not read line")
        }
        counter = counter + 1;
    }

    for i in &comments {
        println!("Comment on line: {}", i);
    }
}

fn is_comment(line: String, mut flag: bool) -> bool {
    let first_chars = &line.trim().get(0..2);
    match first_chars {
        &None => (),
        &Some(chars) => {
            if chars == "//" {
                return true;
            } else if chars == "/*" {
                flag = true;
                return true;
            } else if chars == "*/" && flag == true {
                flag = false;
                return true;
            } else if &chars.get(0..1).unwrap() == &"*" {
                return true;
            }
            return false;
        }
    }
    false
}

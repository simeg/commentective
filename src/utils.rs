pub mod path {
    use std::io::Error;
    use std::io::ErrorKind;
    use std::path::Path;
    use utils::string::s;

    pub fn filename(path: &Path) -> Result<String, Error> {
        match path.file_name() {
            None => Err(Error::new(
                ErrorKind::InvalidData,
                "No OsStr present in input",
            )),
            Some(os_str) => match os_str.to_str() {
                None => Err(Error::new(
                    ErrorKind::InvalidData,
                    "Unable to convert OsStr -> str",
                )),
                Some(str) => Ok(s(str)),
            },
        }
    }
}

pub mod string {
    pub fn string_contains_any_of(input: String, matches: Vec<&str>) -> bool {
        for pattern in matches {
            if input.contains(pattern) {
                return true;
            } else {
                continue;
            }
        }
        false
    }

    pub fn s(input: &str) -> String {
        String::from(input)
    }
}

pub mod list {
    pub fn in_list(needle: &String, haystack: Vec<String>) -> bool {
        match haystack.into_iter().rfind(|ele| ele == needle) {
            None => false,
            Some(_) => true,
        }
    }
}

pub mod comments {
    use std::fs::File;
    use std::io::BufRead;
    use std::io::BufReader;
    use utils::list::in_list;

    #[derive(Debug)]
    pub struct Line {
        index: u32,
        content: String,
    }

    pub fn find_comments(
        starts: Vec<String>,
        ends: Vec<String>,
        file: &File,
        is_single_line_comment: &Fn(&str) -> bool,
    ) -> Vec<u32> {
        let mut comments = Vec::<u32>::new();
        let mut is_multi = false;

        for line in file_to_lines(file) {
            if is_single_line_comment(&line.content) {
                comments.push(line.index);
            } else if in_list(&line.content, starts.clone()) {
                is_multi = true;
                comments.push(line.index);
            } else if in_list(&line.content, ends.clone()) {
                is_multi = false;
                comments.push(line.index);
            } else {
                if is_multi {
                    comments.push(line.index);
                }
            }
        }

        comments
    }

    fn file_to_lines(file: &File) -> Vec<Line> {
        let mut counter: u32 = 1;
        BufReader::new(file)
            .lines()
            .into_iter()
            .map(|line| match line {
                Ok(l) => {
                    let line = Line {
                        index: counter,
                        content: l,
                    };
                    counter = counter + 1;
                    line
                }
                Err(_) => panic!("Could not read line"),
            })
            .collect()
    }
}

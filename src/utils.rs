pub mod path {
    use std::ffi::OsStr;
    use std::ffi::OsString;
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

    pub fn exists_on_filesystem(path: &OsStr) -> Result<(), OsString> {
        match path.to_str() {
            None => Err(OsString::from("Could not convert input file path -> str")),
            Some(p) => {
                if Path::new(p).exists() {
                    return Ok(());
                }
                Err(OsString::from(format!(
                    "Cannot verify that file exist [{}]",
                    path.to_str().unwrap()
                )))
            }
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

    pub fn string_contains_all(input: String, matches: Vec<&str>) -> bool {
        let found_matches = matches
            .clone()
            .into_iter()
            .filter(|m| input.contains(m))
            .collect::<Vec<&str>>();
        found_matches.len() == matches.len()
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
    use utils::string::s;

    #[derive(Debug)]
    pub struct Line {
        index: u32,
        content: String,
    }

    #[derive(Debug)]
    pub struct MultiCommentOpts {
        pub starts: Vec<String>,
        pub ends: Vec<String>,
    }

    pub fn find_comments(
        file: &File,
        multi_opts: &MultiCommentOpts,
        is_single_line_comment: &Fn(&str) -> bool,
    ) -> Vec<u32> {
        let mut comments = Vec::<u32>::new();
        let mut is_multi = false;

        for line in file_to_lines(file) {
            if is_single_line_comment(&s(line.content.trim())) {
                comments.push(line.index);
            } else if in_list(&s(line.content.trim()), multi_opts.starts.clone()) {
                is_multi = true;
                comments.push(line.index);
            } else if in_list(&s(line.content.trim()), multi_opts.ends.clone()) {
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

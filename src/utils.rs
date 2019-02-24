pub mod path {
    use std::ffi::OsStr;
    use std::ffi::OsString;
    use std::io::Error;
    use std::io::ErrorKind;
    use std::path::Path;
    use utils::string::str;

    pub fn filename(path: &Path) -> Result<String, Error> {
        match path.file_name() {
            None => Err(Error::new(
                ErrorKind::InvalidData,
                "No OsStr present in path",
            )),
            Some(os_str) => match os_str.to_str() {
                None => Err(Error::new(
                    ErrorKind::InvalidData,
                    "Unable to convert OsStr -> str",
                )),
                Some(string) => Ok(str(string)),
            },
        }
    }

    pub fn extension(path: &Path) -> Result<String, ()> {
        let extension = path
            .extension()
            .expect("No extension in path")
            .to_str()
            .expect("Could not convert OsStr -> &str");
        Ok(str(extension))
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

    pub fn str(input: &str) -> String {
        String::from(input)
    }

    pub fn first_char(input: &str) -> String { input.chars().skip(0).take(1).collect() }
}

pub mod list {
    pub fn in_list(needle: &String, haystack: Vec<String>) -> bool {
        match haystack.into_iter().rfind(|ele| needle.contains(ele)) {
            None => false,
            Some(_) => true,
        }
    }
}

pub mod comments {
    use language::FindResult;
    use std::fs::File;
    use std::io::BufRead;
    use std::io::BufReader;
    use utils::list::in_list;
    use utils::string::str;

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
            if is_single_line_comment(&str(line.content.trim())) {
                comments.push(line.index);
            } else if in_list(&str(line.content.trim()), multi_opts.starts.clone()) {
                is_multi = true;
                comments.push(line.index);
            } else if in_list(&str(line.content.trim()), multi_opts.ends.clone()) {
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

    pub fn noop_find_result() -> FindResult {
        FindResult {
            file_name: str("SHOULD_NOT_BE_PRINTED"),
            lines: [].to_vec(),
            print: false,
        }
    }
}

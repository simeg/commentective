pub mod path {
    use std::io::Error;
    use std::io::ErrorKind;
    use std::path::Path;

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
                Some(str) => Ok(String::from(str)),
            },
        }
    }
}

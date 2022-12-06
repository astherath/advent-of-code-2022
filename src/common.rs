use std::fs;
use std::{error::Error, fmt};
pub type PathString<'a> = &'a str;
pub type FileData = Vec<String>;
pub type FileIOResult = Result<FileData, FileIOError>;

pub fn read_input(path: PathString) -> FileIOResult {
    Ok(fs::read_to_string(path)?
        .parse::<String>()
        .unwrap()
        .split("\n")
        .map(|x| x.to_string())
        .collect())
}

#[derive(Debug)]
pub struct FileIOError(String);

impl Error for FileIOError {}

impl fmt::Display for FileIOError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "custom IO errror: {}", self.0)
    }
}

impl From<std::io::Error> for FileIOError {
    fn from(error: std::io::Error) -> Self {
        FileIOError(error.to_string())
    }
}

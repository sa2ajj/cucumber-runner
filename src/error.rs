use std::io;

use regex;

#[derive(Debug)]
pub enum Error {
    Misc(String),
    IO(io::Error),
    Regex(regex::Error),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IO(err)
    }
}

impl From<regex::Error> for Error {
    fn from(err: regex::Error) -> Self {
        Error::Regex(err)
    }
}

impl Error {
    pub fn from_str(msg: &str) -> Self {
        Error::Misc(msg.to_owned())
    }
}

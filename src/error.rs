// use std::error;
use std::fmt;
use std::io;

use regex;

#[derive(Debug)]
pub enum Error {
    Misc(String),
    ParseError(String, usize, String),
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

    pub fn parse_error(filename: &str, lineno: usize, msg: &str) -> Error {
        Error::ParseError(filename.to_owned(), lineno, msg.to_owned())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Misc(ref err) =>
                write!(f, "Misc error: {}", err),

            Error::ParseError(ref filename, ref lineno, ref err) =>
                write!(f, "{}: {}: parse error: {}", filename, lineno, err),

            Error::IO(ref err) =>
                err.fmt(f),

            Error::Regex(ref err) =>
                err.fmt(f),
        }
    }
}

// impl error::Error for Error {
//     fn description(&self) -> &str {
//         match *self {
//             Error::Io(ref err) =>
//                 err.description(),
// 
//             Error::Parse(ref err) =>
//                 err.description(),
// 
//             _ => {
//                 format!("some error");
//             }
//         }
//     }
// 
//     fn cause(&self) -> Option<&error::Error> {
//         match *self {
//             // N.B. Both of these implicitly cast `err` from their concrete
//             // types (either `&io::Error` or `&num::ParseIntError`)
//             // to a trait object `&Error`. This works because both error types
//             // implement `Error`.
//             Error::Io(ref err) => Some(err),
//             Error::Parse(ref err) => Some(err),
//         }
//     }
// }

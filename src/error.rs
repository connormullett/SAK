use std::fmt::Display;
use std::io;
use std::str::Utf8Error;

#[derive(Debug)]
pub enum SakError {
    Parse(String),
    InvalidArgument(String),
    IoError(io::Error),
    Utf8Error(Utf8Error),
}

impl From<io::Error> for SakError {
    fn from(err: io::Error) -> Self {
        Self::IoError(err)
    }
}

impl From<Utf8Error> for SakError {
    fn from(err: Utf8Error) -> Self {
        Self::Utf8Error(err)
    }
}

impl Display for SakError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use self::SakError::*;

        match self {
            Parse(value) => write!(f, "could not parse value `{}`", value),
            InvalidArgument(value) => {
                write!(f, "Invalid command `{}`, use --help for usage", value)
            }
            IoError(value) => {
                write!(f, "{}", value)
            }
            Utf8Error(value) => write!(
                f,
                "Data given was invalid UTF-8, cant perform conversion: Error was {}",
                value
            ),
        }
    }
}

use std::{
    fmt::{Display, Formatter, Result},
    string::{FromUtf8Error, FromUtf16Error},
};

pub mod parse;
pub use parse::GVASParseError;

#[derive(Debug)]
pub enum GVASError {
    ParseError(GVASParseError),
    EmptyFileError(String),
    FileIOError(std::io::Error),
    UnexpectedError(&'static str),
}

impl Display for GVASError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            GVASError::ParseError(ref error) => {
                Display::fmt(error, f)
            },
            GVASError::EmptyFileError(ref path) => {
                write!(f, "File is empty: {:?}", path)
            },
            GVASError::FileIOError(ref error) => {
                Display::fmt(error, f)
            },
            GVASError::UnexpectedError(ref message) => {
                write!(f, "An unexpected error occurred")?;
                write!(f, "Message: {}", message)
            }
        }
    }
}

impl std::error::Error for GVASError {}

impl From<GVASParseError> for GVASError {
    fn from(error: GVASParseError) -> Self {
        GVASError::ParseError(error)
    }
}

impl From<std::io::Error> for GVASError {
    fn from(error: std::io::Error) -> Self {
        GVASError::FileIOError(error)
    }
}

impl From<FromUtf8Error> for GVASError {
    fn from(error: FromUtf8Error) -> Self {
        GVASError::ParseError(GVASParseError::InvalidUtf8Conversion(error))
    }
}

impl From<FromUtf16Error> for GVASError {
    fn from(error: FromUtf16Error) -> Self {
        GVASError::ParseError(GVASParseError::InvalidUtf16Conversion(error))
    }
}

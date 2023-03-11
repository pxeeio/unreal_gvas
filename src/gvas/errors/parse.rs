use std::{
    fmt::{Display, Formatter, Result},
    string::{FromUtf8Error, FromUtf16Error},
};

/// Thrown when there is an error while parsing the GVAS file
#[derive(Debug)]
pub enum GVASParseError {
    /// If the first 4 bytes of the file don't match the GVAS file signature
    InvalidFileSignature(u32),
    /// If the parsed string size is invalid
    InvalidUEStringSize(i32),
    /// If the input bytes are not valid UTF-8
    InvalidUtf8Conversion(FromUtf8Error),
    /// If the input bytes not valid UTF-16
    InvalidUtf16Conversion(FromUtf16Error),
}

impl Display for GVASParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            GVASParseError::InvalidFileSignature(ref signature) => {
                write!(f, "Invalid file signature: {:#04X}", signature)
            },
            GVASParseError::InvalidUEStringSize(ref size) => {
                write!(f, "Invalid string size: {}", size)
            },
            GVASParseError::InvalidUtf8Conversion(ref error) => {
                Display::fmt(error, f)
            },
            GVASParseError::InvalidUtf16Conversion(ref error) => {
                Display::fmt(error, f)
            },
        }
    }
}

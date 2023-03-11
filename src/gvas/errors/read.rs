use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum GVASReadError {
    EofError(std::io::Error),
    /// If the parsed string size is invalid
    InvalidUEStringSize(i32),
    UnexpectedError(&'static str),
}

impl Display for GVASReadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            GVASReadError::EofError(ref error) => {
                Display::fmt(error, f)
            },
            GVASReadError::InvalidUEStringSize(ref size) => {
                write!(f, "Invalid string size: {}", size)
            },
            GVASReadError::UnexpectedError(ref message) => {
                write!(f, "An unexpected error occurred")?;
                write!(f, "Message: {}", message)
            }
        }
    }
}

impl std::error::Error for GVASReadError {}

impl From<std::io::Error> for GVASReadError {
    fn from(error: std::io::Error) -> Self {
        match error.kind() {
            std::io::ErrorKind::UnexpectedEof => GVASReadError::EofError(error),
            _ => GVASReadError::UnexpectedError("An unexpected error occurred"),
        }
    }
}

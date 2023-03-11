//! This module contains the main types and functions used for handling GVAS files.
//! 
//! It also provides an `errors` submodule that defines custom error types

mod data;
pub use data::CustomData;

pub mod errors;
pub use errors::{
    GVASError,
    parse::GVASParseError,
};

mod guid;
pub use guid::GUID;

mod gvas;
pub use gvas::GVAS;

mod parser;
pub use parser::GVASParser;

mod properties;

mod reader;

mod version;
pub mod int;

use std::{
    io::{Cursor}
};

use crate::errors::GVASError;

pub trait UEPropertyTrait {
    type Name;
    type Value;

    fn new(&mut self, cursor: Cursor<Vec<u8>>) -> Result<(), GVASError>;
}

pub enum UEProperty {
    Array,
    Bool,
    Byte,
    Double,
    Enum,
    Float,
    Int,
    Int8,
    Int16,
    Int64,
    Map,
    Set,
    Str,
    Struct,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Unknown,
}

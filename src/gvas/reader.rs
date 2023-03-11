use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{Cursor, Read};

use crate::errors::GVASReadError;

pub struct GVASReader {
    cursor: Cursor<Vec<u8>>,
}

impl GVASReader {
    pub fn new(cursor: Cursor<Vec<u8>>) -> Self {
        GVASReader { cursor }
    }

    pub fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), GVASReadError> {
        match self.cursor.read_exact(buf) {
            Ok(_) => Ok(()),
            Err(error) => Err(error.into()),
        }
    }

    pub fn read_i32(&mut self) -> Result<i32, GVASReadError> {
        match self.cursor.read_i32::<LittleEndian>() {
            Ok(value) => Ok(value),
            Err(error) => Err(error.into()),}
    }

    pub fn read_u8(&mut self) -> Result<u8, GVASReadError> {
        match self.cursor.read_u8() {
            Ok(value) => Ok(value),
            Err(error) => Err(error.into()),
        }
    }

    pub fn read_u16(&mut self) -> Result<u16, GVASReadError> {
        match self.cursor.read_u16::<LittleEndian>() {
            Ok(value) => Ok(value),
            Err(error) => Err(error.into()),
        }
    }

    pub fn read_u32(&mut self) -> Result<u32, GVASReadError> {
        match self.cursor.read_u32::<LittleEndian>() {
            Ok(value) => Ok(value),
            Err(error) => Err(error.into()),
        }
    }

    pub fn read_u64(&mut self) -> Result<u64, GVASReadError> {
        match self.cursor.read_u64::<LittleEndian>() {
            Ok(value) => Ok(value),
            Err(error) => Err(error.into()),
        }
    }

    pub fn read_ue_string(&mut self) -> Result<Option<String>, GVASReadError> {
        let length: i32 = self.read_i32()?;

        if length == 0 {
            return Ok(None);
        } else if length < 0 {
            return Err(GVASReadError::InvalidUEStringSize(length));
        } else if length == 1 {
            return Ok(Some(String::from("")));
        } else {
            let mut bytes: Vec<u8> = vec![0u8; length as usize];
            self.read_exact(&mut bytes)?;
    
            Ok(Some(
                String::from_utf8(bytes).unwrap_or_else(|_| String::from("None"))
            ))
        }
    }
}

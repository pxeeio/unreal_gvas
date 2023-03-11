use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;

use crate::errors::GVASError;

pub struct UEIntProperty {
    pub value: i32,
}

impl UEIntProperty {
    pub fn new(cursor: &mut Cursor<Vec<u8>>) -> Result<Self, GVASError> {
        let value: i32 = cursor.read_i32::<LittleEndian>()?;

        Ok(UEIntProperty { value })
    }
}

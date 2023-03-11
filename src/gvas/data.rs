use std::{
    fmt::{Display, Formatter, Result},
};

use super::guid::GUID;

pub struct GameData {
    pub version: u32,
    pub count: usize,
    pub data: Vec<CustomData>,
}

/// Custom game data stored in the GVAS file
pub struct CustomData {
    pub guid: GUID,
    pub value: i32,
}

impl Display for GameData {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "Data Version: {}", self.version)?;
        writeln!(f, "Data Count: {}", self.count)?;

        // for data in &self.data {
        //     Display::fmt(data, f)?;
        // }

        Ok(())
    }
}

impl Display for CustomData {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "GUID: {}", self.guid)?;
        writeln!(f, "Value: {}", self.value)
    }
}

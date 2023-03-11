use std::{
    fmt::{Display, Formatter, Result},
};

use super::{
    data::GameData,
    version::UEngineVersion,
};

pub struct GVAS {
    /// GVAS file version
    pub save_game_file_version: i32,
    /// GVAS system version
    pub package_file_ue4_version: i32,
    /// Unreal Engine version information
    pub engine_version: UEngineVersion,
    /// The file cursor for the GVAS file
    // pub cursor: Cursor<Vec<u8>>,
    /// The size of the GVAS file in bytes
    pub size: u64,
    /// The custom data stored in the GVAS file
    pub game_data: GameData,

    pub save_game_type: String,
}

impl Display for GVAS {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "{}", self.engine_version)?;
        writeln!(f, "Package File UE4 Version: {}", self.package_file_ue4_version)?;
        writeln!(f, "Save Game Type: {}", self.save_game_type)?;
        writeln!(f, "Save Game File Version: {}", self.save_game_file_version)?;
        writeln!(f, "File Size: {} bytes", self.size)?;
        write!(f, "{}", self.game_data)
    }
}

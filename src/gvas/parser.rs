use std::{
    fs::{File, Metadata},
    io::{Cursor, Read},
    path::Path,
};

use super::{
    data::{CustomData, GameData},
    errors::{
        GVASError,
        GVASParseError,
        GVASReadError,
    },
    guid::GUID,
    GVAS,
    reader::GVASReader,
    version::UEngineVersion,
};

pub struct GVASParser {}

impl GVASParser {
    /// GVAS file signature in Little Endian. Decodes to "GVAS"
    const GVAS_FILE_SIGNATURE: u32 = 0x53415647;

    fn open_file<P>(path: &P) -> Result<(GVASReader, u64), GVASError> where P: AsRef<Path> {
        let mut file: File = File::open(path)?;
        let meta: Metadata = file.metadata()?;

        if meta.len() == 0 {
            return match path.as_ref().to_str() {
                Some(path) => Err(GVASError::EmptyFileError(path.to_string())),
                None => Err(GVASError::UnexpectedError("Failed to convert path to string")),
            };
        }

        let mut bytes: Vec<u8> = Vec::new();
        file.read_to_end(&mut bytes)?;

        let mut cursor: Cursor<Vec<u8>> = Cursor::new(bytes);

        Ok((GVASReader::new(cursor), meta.len()))
    }

    fn validate_file_signature(reader: &mut GVASReader) -> Result<(), GVASError> {
        let file_signature: u32 = reader.read_u32()?;

        if file_signature != Self::GVAS_FILE_SIGNATURE {
            return Err(GVASParseError::InvalidFileSignature(file_signature).into());
        }

        Ok(())
    }

    /// Given an absolute path to a file, validates and parses the file into a GVAS struct
    pub fn parse<P>(path: &P) -> Result<GVAS, GVASError> where P: AsRef<Path> {
        let result: (GVASReader, u64) = Self::open_file(path)?;
        let mut reader: GVASReader = result.0;
        let size: u64 = result.1;

        Self::validate_file_signature(&mut reader)?;

        let save_game_file_version: i32 = reader.read_i32()?;
        let package_file_ue4_version: i32 = reader.read_i32()?;

        let major: u16 = reader.read_u16()?;
        let minor: u16 = reader.read_u16()?;
        let patch: u16 = reader.read_u16()?;
        let change_list: u32 = reader.read_u32()?;
        let branch: String = match reader.read_ue_string() {
            Ok(branch) => branch.unwrap(),
            Err(error) => {
                return Err(error.into());
            }
        };
        
        let engine_version: UEngineVersion = UEngineVersion {
            major,
            minor,
            patch,
            changelist: change_list,
            branch,
        };

        let game_data_version: u32 = reader.read_u32()?;
        let game_data_count: usize = reader.read_u32()? as usize;
        let mut game_data_values: Vec<CustomData> = Vec::with_capacity(game_data_count);

        for _ in 0..game_data_count {
            let mut guid = [0u8; 16];
            reader.read_exact(&mut guid)?;

            let value = reader.read_i32()?;

            let data: CustomData = CustomData {
                guid: GUID::new(guid),
                value,
            };

            game_data_values.push(data);
        }

        let game_data: GameData = GameData {
            version: game_data_version,
            count: game_data_count,
            data: game_data_values,
        };

        let save_game_type: String = reader.read_ue_string().unwrap().unwrap(); 

        Ok(GVAS {
            save_game_file_version,
            package_file_ue4_version,
            engine_version,
            size,
            game_data,
            save_game_type,
        })
    }
}

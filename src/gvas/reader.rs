use byteorder::{LittleEndian, ReadBytesExt};
use std::{
    fs::{File, Metadata},
    io::{Cursor, Read, Seek},
    path::Path,
};

use super::{
    errors::{
        GVASError,
        GVASParseError,
    },
    GVAS,
    UEngineVersion,
};

pub struct GVASReader {}

impl GVASReader {
    /// GVAS file signature in Little Endian. Decodes to "GVAS"
    const GVAS_FILE_SIGNATURE: i32 = 0x53415647;

    fn open_file<P>(path: &P) -> Result<(Cursor<Vec<u8>>, u64), GVASError> where P: AsRef<Path> {
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

        Ok((Cursor::new(bytes), meta.len()))
    }

    fn validate_file_signature(cursor: &mut Cursor<Vec<u8>>) -> Result<(), GVASError> {
        let file_signature: i32 = cursor.read_i32::<LittleEndian>()?;

        if file_signature != Self::GVAS_FILE_SIGNATURE {
            return Err(GVASParseError::InvalidFileSignature(file_signature).into());
        }

        Ok(())
    }

    /// Given an absolute path to a file, validates and parses the file into a GVAS struct
    pub fn parse<P>(path: &P) -> Result<GVAS, GVASError> where P: AsRef<Path> {
        let result: (Cursor<Vec<u8>>, u64) = Self::open_file(path)?;
        let mut cursor: Cursor<Vec<u8>> = result.0;
        let size: u64 = result.1;

        Self::validate_file_signature(&mut cursor)?;

        let save_game_file_version: i32 = cursor.read_i32::<LittleEndian>()?;
        let package_file_ue4_version: i32 = cursor.read_i32::<LittleEndian>()?;

        let major: u16 = cursor.read_u16::<LittleEndian>()?;
        let minor: u16 = cursor.read_u16::<LittleEndian>()?;
        let patch: u16 = cursor.read_u16::<LittleEndian>()?;
        let change_list: u32 = cursor.read_u32::<LittleEndian>()?;
        let branch_len: i32 = cursor.read_i32::<LittleEndian>()?;     

        if branch_len <= 0 {
            return Err(GVASParseError::InvalidUEStringSize(branch_len).into());
        }   

        // Do not read the null terminator
        let mut buff: Vec<u8> = vec![0u8; branch_len as usize - 1];
        cursor.read_exact(&mut buff)?;

        let branch: String = String::from_utf8(buff)?;
        
        let engine_version: UEngineVersion = UEngineVersion {
            major,
            minor,
            patch,
            changelist: change_list,
            branch: branch,
        };

        cursor.rewind()?;

        Ok(GVAS {
            save_game_file_version,
            package_file_ue4_version,
            engine_version,
            cursor,
            size,
        })        
    }
}

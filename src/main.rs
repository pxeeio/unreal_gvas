use std::path::Path;

mod gvas;
use crate::gvas::{GVAS, GVASError, GVASReader};

fn main() {
    let path = Path::new(
        env!("CARGO_MANIFEST_DIR")
    ).join("tests/resources/HL-00-00.sav");

    let gvas: Result<GVAS, GVASError> = GVASReader::parse(&path);

    match gvas {
        Ok(gvas) => println!("{}", gvas),
        Err(error) => println!("Error: {}", error),
    }
}
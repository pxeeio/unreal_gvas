use std::path::Path;

mod gvas;
use crate::gvas::{GVASError, GVASReader};

fn main() -> Result<(), GVASError> {
    let path = Path::new(
        env!("CARGO_MANIFEST_DIR")
    ).join("resources/HL-00-00.sav");

    let gvas = GVASReader::parse(&path)?;

    println!("{}", gvas);

    Ok(())
}
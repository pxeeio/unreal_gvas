use std::path::Path;

use unreal_gvas::{
    GVAS,
    GVASParser,
    errors::GVASError,
};

#[test]
fn test_parse_success() {
    let path = Path::new(
        env!("CARGO_MANIFEST_DIR")
    ).join("tests/resources/HL-00-00.sav");

    let gvas: Result<GVAS, GVASError> = GVASParser::parse(&path);

    match gvas {
        Ok(gvas) => println!("{}", gvas),
        Err(error) => println!("Error: {}", error),
    }
}
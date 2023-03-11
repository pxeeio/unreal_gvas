use std::{
    fs::{remove_file, write},
    path::Path,
};

use unreal_gvas::{
    GVAS,
    GVASParser,
    errors::GVASError,
};

#[test]
fn test_error_on_nonexistent_file() {
    let path = Path::new(
        env!("CARGO_MANIFEST_DIR")
    ).join("tests/resources/i-dont-exist.sav");

    let gvas: Result<GVAS, GVASError> = GVASParser::parse(&path);
    assert!(gvas.is_err());
}

#[test]
fn test_error_on_empty_gvas_file() {
    let path = Path::new(
        env!("CARGO_MANIFEST_DIR")
    ).join("empty.sav");

    write(&path, "").unwrap();

    let gvas: Result<GVAS, GVASError> = GVASParser::parse(&path);

    // Clean up here in case this test fails
    remove_file(path).unwrap();

    assert!(gvas.is_err());
}

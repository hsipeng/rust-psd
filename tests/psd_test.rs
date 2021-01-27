use failure::Error;
use std::include_bytes;
use rust_psd::Psd;

#[test]
fn file_header_section() -> Result<(), Error> {
    let psd = include_bytes!("./green-1x1.psd");

    let psd = Psd::from_bytes(psd)?;

    assert_eq!(psd.width(), 1);
    assert_eq!(psd.height(), 1);

    Ok(())
}
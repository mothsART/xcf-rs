use std::fs::File;
use std::io:: Write;
use sha1::{Digest, Sha1};

use xcf::data::{color::ColorType, error::Error};
use xcf::create::XcfCreator;

fn assert_hash(path: &'static str, expected_hash: &'static str) {
    let bytes = std::fs::read(&path).unwrap();
    let mut hasher = Sha1::new();
    hasher.update(&bytes);
    let hash = format!("{:x}", hasher.finalize());
    assert_eq!(expected_hash, hash);
}

#[test]
fn write_minimal_xcf1() -> Result<(), Error> {
    let path = "tests/samples/minimal_xcf1.xcf";
    let mut minimal_xcf = File::create(path)?;
    let mut xcf = XcfCreator::new(1, 1, 1, ColorType::Rgb);
    xcf.add_properties();
    xcf.add_layers();
    minimal_xcf.write_all(xcf.data.as_slice())?;

    assert_hash(path, "c67e130f2c298b1241342c620f28233b0828b2bb");
    Ok(())
}

#[test]
fn write_minimal_xcf3() -> Result<(), Error> {
    let path = "tests/samples/minimal_xcf3.xcf";
    let mut minimal_xcf = File::create(path)?;
    let mut xcf = XcfCreator::new(3, 1, 1, ColorType::Rgb);
    xcf.add_properties();
    xcf.add_layers();
    minimal_xcf.write_all(xcf.data.as_slice())?;

    assert_hash(path, "89e270d75b5fef87d222c3dabf2fc800aeb4ac2d");
    Ok(())
}

#[test]
fn write_minimal_xcf10() -> Result<(), Error> {
    let path = "tests/samples/minimal_xcf10.xcf";
    let mut minimal_xcf = File::create(path)?;
    let mut xcf = XcfCreator::new(10, 1, 1, ColorType::Rgb);
    xcf.add_properties();
    xcf.add_layers();
    minimal_xcf.write_all(xcf.data.as_slice())?;

    assert_hash(path, "20260eefdc4401cd41cb9e8c3ebc044cd4f1029f");
    Ok(())
}

#[test]
fn write_minimal() -> Result<(), Error> {
    let mut minimal_xcf = File::create("tests/samples/minimal.xcf")?;
    let mut xcf = XcfCreator::new(11, 1, 1, ColorType::Rgb);
    xcf.add_properties();
    xcf.add_layers();
    minimal_xcf.write_all(xcf.data.as_slice())?;
    Ok(())
}
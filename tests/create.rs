use std::fs::File;
use std::io:: Write;
use sha1::{Digest, Sha1};

use xcf::data::{color::ColorType, error::Error, property::{Property, PropertyIdentifier, PropertyPayload}};
use xcf::data::xcf::XcfCompression;
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
    let properties = vec!();
    xcf.add_properties(&properties);
    xcf.add_layers();
    minimal_xcf.write_all(xcf.data.as_slice())?;

    assert_hash(path, "9e54fb4fc2658de528398a66cc684ada35866807");
    Ok(())
}

#[test]
fn write_minimal_xcf3() -> Result<(), Error> {
    let path = "tests/samples/minimal_xcf3.xcf";
    let mut minimal_xcf = File::create(path)?;
    let mut xcf = XcfCreator::new(3, 1, 1, ColorType::Rgb);
    let properties = vec!();
    xcf.add_properties(&properties);
    xcf.add_layers();
    minimal_xcf.write_all(xcf.data.as_slice())?;

    assert_hash(path, "1b9d7187a9b783cd3ce16790ab1ebe7a05eac119");
    Ok(())
}

#[test]
fn write_minimal_xcf10() -> Result<(), Error> {
    let path = "tests/samples/minimal_xcf10.xcf";
    let mut minimal_xcf = File::create(path)?;
    let mut xcf = XcfCreator::new(10, 1, 1, ColorType::Rgb);
    let properties = vec!();
    xcf.add_properties(&properties);
    xcf.add_layers();
    minimal_xcf.write_all(xcf.data.as_slice())?;

    assert_hash(path, "72dbe0106f48fb25d0fd047acf519f13a3dff086");
    Ok(())
}

#[test]
fn write_minimal() -> Result<(), Error> {
    let mut minimal_xcf = File::create("tests/samples/minimal.xcf")?;
    let mut xcf = XcfCreator::new(11, 1, 1, ColorType::Rgb);

    let mut properties = vec!();
    let compression_property = Property {
        kind: PropertyIdentifier::PropCompression,
        length: 1,
        payload: PropertyPayload::Compression(XcfCompression::None)
    };
    properties.push(compression_property);

    xcf.add_properties(&properties);
    xcf.add_layers();
    minimal_xcf.write_all(xcf.data.as_slice())?;
    Ok(())
}
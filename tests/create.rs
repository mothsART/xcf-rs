use std::fs::File;
use std::io:: Write;
use sha1::{Digest, Sha1};

use xcf::data::{color::ColorType, error::Error, property::{Property, PropertyIdentifier, PropertyPayload}};
use xcf::data::xcf::XcfCompression;
use xcf::create::XcfCreator;
use xcf::data::property::{ResolutionProperty, ParasiteProperty};

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
fn write_minimal_xcf11() -> Result<(), Error> {
    let path = "tests/samples/minimal_xcf11.xcf";
    let mut minimal_xcf = File::create(path)?;
    let mut xcf = XcfCreator::new(11, 1, 1, ColorType::Rgb);
    let properties = vec!();
    xcf.add_properties(&properties);
    xcf.add_layers();
    minimal_xcf.write_all(xcf.data.as_slice())?;

    assert_hash(path, "d3f72da31db4e7e7e474aee624038474bec700ea");
    Ok(())
}

#[test]
fn write_minimal_xcf11_properties() -> Result<(), Error> {
    let path = "tests/samples/minimal_xcf11_properties.xcf";
    let mut minimal_xcf = File::create(path)?;
    let mut xcf = XcfCreator::new(11, 1, 1, ColorType::Rgb);

    let mut properties = vec!();
    let resolution_property = Property {
        kind: PropertyIdentifier::PropResolution,
        length: 8,
        payload: PropertyPayload::ResolutionProperty(ResolutionProperty {
            xres: 300.0,
            yres:  300.0
        })
    };
    properties.push(resolution_property);

    let tattoo_property = Property {
        kind: PropertyIdentifier::PropTattoo,
        length: 4,
        payload: PropertyPayload::Tatoo(2)
    };
    properties.push(tattoo_property);

    let tattoo_property = Property {
        kind: PropertyIdentifier::PropUnit,
        length: 4,
        payload: PropertyPayload::Unit(1)
    };
    properties.push(tattoo_property);

    /*
    let parasites_property = Property {
        kind: PropertyIdentifier::PropParasites,
        length: 238,
        payload: PropertyPayload::Parasites(vec!(
            ParasiteProperty {
                name: "gimp-comment".to_string(),
                flags: 1,
                data: "Test Comment".to_string()
            },
            ParasiteProperty {
                name: "gimp-image-grid".to_string(),
                flags: 1,
                data: "blabla".to_string()
            },            
        ))
    };
    properties.push(parasites_property);
    */
    
    xcf.add_properties(&properties);

    xcf.add_layers();
    minimal_xcf.write_all(xcf.data.as_slice())?;

    assert_hash(path, "c70bf55ffa024604eb0942bdc853ed137f8163ed");
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
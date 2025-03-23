use std::fs::File;
use std::io:: Write;
use sha1::{Digest, Sha1};

use xcf::{data::{color::ColorType, error::Error, property::{Property, PropertyIdentifier, PropertyPayload}, rgba::RgbaPixel}, LayerColorType};
use xcf::data::xcf::XcfCompression;
use xcf::create::XcfCreator;
use xcf::data::property::{ResolutionProperty, ParasiteProperty};
use xcf::data::layer::Layer;
use xcf::data::pixeldata::PixelData;

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
    xcf.add_layers(&vec!());
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
    xcf.add_layers(&vec!());
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
    xcf.add_layers(&vec!());
    minimal_xcf.write_all(xcf.data.as_slice())?;

    assert_hash(path, "72dbe0106f48fb25d0fd047acf519f13a3dff086");
    Ok(())
}

#[test]
fn write_minimal_xcf11() -> Result<(), Error> {
    let path = "tests/samples/minimal_xcf11.xcf";
    let mut minimal_xcf = File::create(path)?;
    let mut xcf = XcfCreator::new(11, 1, 1, ColorType::Rgb);
    xcf.add_properties(&vec!());

    let mut layers = vec!();
    let pixels = vec![
        RgbaPixel::new(158, 36, 222, 0),
    ];
    let pixels_layer_one: PixelData = PixelData { width: 1, height: 1, pixels: pixels };
    let properties_layer_one = vec!();
    let layer_one = Layer {
        width: 1,
        height: 1,
        kind: LayerColorType {
            kind: ColorType::Rgb,
            alpha: true
        },
        name: "Background".to_string(),
        pixels: pixels_layer_one,
        properties: properties_layer_one
    };
    layers.push(layer_one);
    xcf.add_layers(&layers);
    minimal_xcf.write_all(xcf.data.as_slice())?;

    assert_hash(path, "6d6e2decc5c6393e83c6ac255e99fdf6617c4a95");
    Ok(())
}

#[test]
fn write_minimal_xcf11_properties() -> Result<(), Error> {
    let path = "tests/samples/minimal_xcf11_properties.xcf";
    let mut minimal_xcf = File::create(path)?;
    let mut xcf = XcfCreator::new(11, 1, 1, ColorType::Rgb);

    let properties = vec![
        Property {
            kind: PropertyIdentifier::PropCompression,
            length: 1,
            payload: PropertyPayload::Compression(XcfCompression::Rle)
        },
        Property {
            kind: PropertyIdentifier::PropResolution,
            length: 8,
            payload: PropertyPayload::ResolutionProperty(ResolutionProperty {
                xres: 300.0,
                yres:  300.0
            })
        },
        Property {
            kind: PropertyIdentifier::PropTattoo,
            length: 4,
            payload: PropertyPayload::Tatoo(2)
        },
        Property {
            kind: PropertyIdentifier::PropUnit,
            length: 4,
            payload: PropertyPayload::Unit(1)
        },
        Property {
            kind: PropertyIdentifier::PropParasites,
            length: 0,
            payload: PropertyPayload::Parasites(vec![
                ParasiteProperty {
                    name: "gimp-comment".to_string(),
                    flags: 1,
                    data: "Test Comment".to_string()
                },
                ParasiteProperty {
                    name: "gimp-image-grid".to_string(),
                    flags: 1,
                    data: "(style solid)\n(fgcolor (color-rgba 0 0 0 1))\n(bgcolor (color-rgba 1 1 1 1))\n(xspacing 10)\n(yspacing 10)\n(spacing-unit inches)\n(xoffset 0)\n(yoffset 0)\n(offset-unit inches)\n".to_string()
                }
            ])
        }
    ];
    xcf.add_properties(&properties);

    let mut layers = vec!();
    let pixels = vec![
        RgbaPixel::new(158, 36, 222, 0),
    ];
    let pixels_layer_one: PixelData = PixelData { width: 1, height: 1, pixels: pixels };
    let properties_layer_one = vec![
        Property {
            kind: PropertyIdentifier::PropActiveLayer,
            length: 0,
            payload: PropertyPayload::ActiveLayer()
        },
        Property {
            kind: PropertyIdentifier::PropOpacity,
            length: 4,
            payload: PropertyPayload::OpacityLayer(RgbaPixel::new(0, 0, 0, 255))
        },
        Property {
            kind: PropertyIdentifier::PropFloatOpacity,
            length: 4,
            payload: PropertyPayload::FloatOpacityLayer()
        },
        Property {
            kind: PropertyIdentifier::PropVisible,
            length: 4,
            payload: PropertyPayload::VisibleLayer()
        },
        Property {
            kind: PropertyIdentifier::PropLinked,
            length: 4,
            payload: PropertyPayload::LinkedLayer(0)
        },
        Property {
            kind: PropertyIdentifier::PropColorTag,
            length: 4,
            payload: PropertyPayload::ColorTagLayer(0)
        },
        Property {
            kind: PropertyIdentifier::PropLockContent,
            length: 4,
            payload: PropertyPayload::LockContentLayer(0)
        },
        Property {
            kind: PropertyIdentifier::PropLockAlpha,
            length: 4,
            payload: PropertyPayload::LockAlphaLayer(0)
        },
        Property {
            kind: PropertyIdentifier::PropLockPosition,
            length: 4,
            payload: PropertyPayload::LockPositionLayer(0)
        },
        Property {
            kind: PropertyIdentifier::PropApplyMask,
            length: 4,
            payload: PropertyPayload::ApplyMaskLayer(0)
        },
        Property {
            kind: PropertyIdentifier::PropEditMask,
            length: 4,
            payload: PropertyPayload::EditMaskLayer(0)
        },
        Property {
            kind: PropertyIdentifier::PropShowMask,
            length: 4,
            payload: PropertyPayload::ShowMaskLayer(0)
        },
        Property {
            kind: PropertyIdentifier::PropOffsets,
            length: 8,
            payload: PropertyPayload::OffsetsLayer(0, 0)
        },
        Property {
            kind: PropertyIdentifier::PropMode,
            length: 4,
            payload: PropertyPayload::ModeLayer(28) // mode normal after version 10
        },
        Property {
            kind: PropertyIdentifier::PropBlendSpace,
            length: 4,
            payload: PropertyPayload::BlendSpaceLayer(0)
        },
        Property {
            kind: PropertyIdentifier::PropCompositeSpace,
            length: 4,
            payload: PropertyPayload::CompositeSpaceLayer(u32::MAX)
        },
        Property {
            kind: PropertyIdentifier::PropCompositeMode,
            length: 4,
            payload: PropertyPayload::CompositeModeLayer(u32::MAX)
        },
        Property {
            kind: PropertyIdentifier::PropTattoo,
            length: 4,
            payload: PropertyPayload::Tatoo(2)
        },
    ];
    let layer_one = Layer {
        width: 1,
        height: 1,
        kind: LayerColorType {
            kind: ColorType::Rgb,
            alpha: true
        },
        name: "Background".to_string(),
        pixels: pixels_layer_one,
        properties: properties_layer_one
    };
    layers.push(layer_one);
    xcf.add_layers(&layers);
    minimal_xcf.write_all(xcf.data.as_slice())?;

    assert_hash(path, "6d6e2decc5c6393e83c6ac255e99fdf6617c4a95");
    Ok(())
}

#[test]
fn write_minimal() -> Result<(), Error> {
    let mut minimal_xcf = File::create("tests/samples/minimal.xcf")?;
    let mut xcf = XcfCreator::new(11, 1, 1, ColorType::Rgb);

    let mut layers = vec!();
    let pixels = vec![
        RgbaPixel::new(255, 0, 0, 0),
    ];
    let pixels_layer_one: PixelData = PixelData { width: 1, height: 1, pixels: pixels };
    let layer_one = Layer {
        width: 1,
        height: 1,
        kind: LayerColorType {
            kind: ColorType::Rgb,
            alpha: true
        },
        name: "Background".to_string(),
        pixels: pixels_layer_one,
        properties: vec!()
    };
    layers.push(layer_one);
    xcf.add_layers(&layers);
    minimal_xcf.write_all(xcf.data.as_slice())?;
    Ok(())
}

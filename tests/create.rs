use sha1::{Digest, Sha1};
use std::fs::{create_dir, File};
use std::path::{Path, PathBuf};
use std::io::Write;

use xcf_rs::create::XcfCreator;
use xcf_rs::data::layer::Layer;
use xcf_rs::data::pixeldata::PixelData;
use xcf_rs::data::property::{ParasiteProperty, ResolutionProperty};
use xcf_rs::data::xcf::XcfCompression;
use xcf_rs::{
    data::{
        color::ColorType,
        error::Error,
        property::{Property, PropertyIdentifier, PropertyPayload},
        rgba::RgbaPixel,
    },
    LayerColorType,
    LayerColorValue,
};

fn assert_hash(path: &str, expected_hash: &str) {
    let bytes = std::fs::read(&path).unwrap();
    let mut hasher = Sha1::new();
    hasher.update(&bytes);
    let hash = format!("{:x}", hasher.finalize());
    assert_eq!(expected_hash, hash);
}

fn create_file(file_name: &'static str, xcf: &mut XcfCreator)-> Result<(File, PathBuf), Error> {
    let dest_dir = Path::new("tests/samples/create");

    if let Err(_e) = create_dir(dest_dir) {}

    let path = dest_dir.join(file_name);
    let new_file = xcf.save(&path)?;
    Ok((new_file, path))
}

#[test]
fn write_minimal_xcf1() -> Result<(), Error> {
    let mut xcf = XcfCreator::new(1, 1, 1, ColorType::Rgb);
    let properties = vec![];
    xcf.add_properties(&properties);
    xcf.add_layers(&vec![]);
    let xcf_file = create_file("minimal_xcf1.xcf", &mut xcf)?;
    assert_hash(xcf_file.1.to_str().expect(""), "9e54fb4fc2658de528398a66cc684ada35866807");
    Ok(())
}

#[test]
fn write_minimal_xcf3() -> Result<(), Error> {
    let mut xcf = XcfCreator::new(3, 1, 1, ColorType::Rgb);
    let properties = vec![];
    xcf.add_properties(&properties);
    xcf.add_layers(&vec![]);
    let xcf_file = create_file("minimal_xcf3.xcf", &mut xcf)?;
    assert_hash(xcf_file.1.to_str().expect(""), "1b9d7187a9b783cd3ce16790ab1ebe7a05eac119");
    Ok(())
}

#[test]
fn write_minimal_xcf10() -> Result<(), Error> {
    let mut xcf = XcfCreator::new(10, 1, 1, ColorType::Rgb);
    let properties = vec![];
    xcf.add_properties(&properties);
    xcf.add_layers(&vec![]);
    let xcf_file = create_file("minimal_xcf10.xcf", &mut xcf)?;
    assert_hash(xcf_file.1.to_str().expect(""), "72dbe0106f48fb25d0fd047acf519f13a3dff086");
    Ok(())
}

#[test]
fn write_minimal_xcf11() -> Result<(), Error> {
    let mut xcf = XcfCreator::new(11, 1, 1, ColorType::Rgb);
    xcf.add_properties(&vec![]);

    let mut layers = vec![];
    let pixels = vec![RgbaPixel::new(158, 36, 222, 0)];
    let pixels_layer_one: PixelData = PixelData {
        width: 1,
        height: 1,
        pixels: pixels,
    };
    let properties_layer_one = vec![];
    let layer_one = Layer {
        width: 1,
        height: 1,
        kind: LayerColorType {
            kind: LayerColorValue::Rgb,
            alpha: false, // TODO : delete ? LayerColorValue can determine alpha value
        },
        name: "Background".to_string(),
        pixels: pixels_layer_one,
        properties: properties_layer_one,
    };
    layers.push(layer_one);
    xcf.add_layers(&layers);
    let xcf_file = create_file("minimal_xcf11.xcf", &mut xcf)?;
    assert_hash(xcf_file.1.to_str().expect(""), "6d6e2decc5c6393e83c6ac255e99fdf6617c4a95");
    Ok(())
}

#[test]
fn write_minimal_xcf11_properties() -> Result<(), Error> {
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

    let mut layers = vec![];
    let pixels = vec![RgbaPixel::new(158, 36, 222, 0)];
    let pixels_layer_one: PixelData = PixelData {
        width: 1,
        height: 1,
        pixels: pixels,
    };
    let properties_layer_one = vec![
        Property {
            kind: PropertyIdentifier::PropActiveLayer,
            length: 0,
            payload: PropertyPayload::ActiveLayer(),
        },
        Property {
            kind: PropertyIdentifier::PropOpacity,
            length: 4,
            payload: PropertyPayload::OpacityLayer(RgbaPixel::new(0, 0, 0, 255)),
        },
        Property {
            kind: PropertyIdentifier::PropFloatOpacity,
            length: 4,
            payload: PropertyPayload::FloatOpacityLayer(),
        },
        Property {
            kind: PropertyIdentifier::PropVisible,
            length: 4,
            payload: PropertyPayload::VisibleLayer(),
        },
        Property {
            kind: PropertyIdentifier::PropLinked,
            length: 4,
            payload: PropertyPayload::LinkedLayer(0),
        },
        Property {
            kind: PropertyIdentifier::PropColorTag,
            length: 4,
            payload: PropertyPayload::ColorTagLayer(0),
        },
        Property {
            kind: PropertyIdentifier::PropLockContent,
            length: 4,
            payload: PropertyPayload::LockContentLayer(0),
        },
        Property {
            kind: PropertyIdentifier::PropLockAlpha,
            length: 4,
            payload: PropertyPayload::LockAlphaLayer(0),
        },
        Property {
            kind: PropertyIdentifier::PropLockPosition,
            length: 4,
            payload: PropertyPayload::LockPositionLayer(0),
        },
        Property {
            kind: PropertyIdentifier::PropApplyMask,
            length: 4,
            payload: PropertyPayload::ApplyMaskLayer(0),
        },
        Property {
            kind: PropertyIdentifier::PropEditMask,
            length: 4,
            payload: PropertyPayload::EditMaskLayer(0),
        },
        Property {
            kind: PropertyIdentifier::PropShowMask,
            length: 4,
            payload: PropertyPayload::ShowMaskLayer(0),
        },
        Property {
            kind: PropertyIdentifier::PropOffsets,
            length: 8,
            payload: PropertyPayload::OffsetsLayer(0, 0),
        },
        Property {
            kind: PropertyIdentifier::PropMode,
            length: 4,
            payload: PropertyPayload::ModeLayer(28), // mode normal after version 10
        },
        Property {
            kind: PropertyIdentifier::PropBlendSpace,
            length: 4,
            payload: PropertyPayload::BlendSpaceLayer(0),
        },
        Property {
            kind: PropertyIdentifier::PropCompositeSpace,
            length: 4,
            payload: PropertyPayload::CompositeSpaceLayer(u32::MAX),
        },
        Property {
            kind: PropertyIdentifier::PropCompositeMode,
            length: 4,
            payload: PropertyPayload::CompositeModeLayer(u32::MAX),
        },
        Property {
            kind: PropertyIdentifier::PropTattoo,
            length: 4,
            payload: PropertyPayload::Tatoo(2),
        },
    ];
    let layer_one = Layer {
        width: 1,
        height: 1,
        kind: LayerColorType {
            kind: LayerColorValue::Rgb,
            alpha: true,
        },
        name: "Background".to_string(),
        pixels: pixels_layer_one,
        properties: properties_layer_one,
    };
    layers.push(layer_one);
    xcf.add_layers(&layers);
    let xcf_file = create_file("minimal_xcf11_properties.xcf", &mut xcf)?;
    assert_hash(xcf_file.1.to_str().expect(""), "6d6e2decc5c6393e83c6ac255e99fdf6617c4a95");
    Ok(())
}

#[test]
fn write_minimal_four_pixels() -> Result<(), Error> {
    let mut xcf = XcfCreator::new(11, 2, 2, ColorType::Rgb);
    xcf.add_properties(&vec![]);
    let mut layers = vec![];
    let pixels = vec![
        RgbaPixel::new(158, 0, 0, 0),   //  #ff0000
        RgbaPixel::new(0, 0, 158, 0),   // #0000ff
        RgbaPixel::new(255, 114, 5, 0), // #ff7205
        RgbaPixel::new(43, 121, 34, 0), // #2b7922
    ];
    let pixels_layer_one: PixelData = PixelData {
        width: 2,
        height: 2,
        pixels: pixels,
    };
    let layer_one = Layer {
        width: 2,
        height: 2,
        kind: LayerColorType {
            kind: LayerColorValue::Rgb,
            alpha: true,
        },
        name: "Background".to_string(),
        pixels: pixels_layer_one,
        properties: vec![],
    };
    layers.push(layer_one);
    xcf.add_layers(&layers);
    let xcf_file = create_file("minimal_four_pixels.xcf", &mut xcf)?;
    assert_hash(xcf_file.1.to_str().expect(""), "8c4c60c226cd932f4c93dff6ce9ccdc3acc7fbde");
    Ok(())
}

#[test]
fn write_minimal_nine_pixels() -> Result<(), Error> {
    let mut xcf = XcfCreator::new(11, 3, 3, ColorType::Rgb);
    xcf.add_properties(&vec![]);
    let mut layers = vec![];
    let pixels = vec![
        RgbaPixel::new(158, 36, 222, 0),  // #9e24de
        RgbaPixel::new(130, 222, 36, 0),  // #82de24
        RgbaPixel::new(222, 36, 36, 0),   // #de2424
        RgbaPixel::new(36, 108, 222, 0),  // #246cde
        RgbaPixel::new(222, 208, 36, 0),  // #ded024
        RgbaPixel::new(5, 97, 48, 0),     // #056130
        RgbaPixel::new(0, 0, 0, 0),       // #000000
        RgbaPixel::new(136, 231, 219, 0), // #88e7db
        RgbaPixel::new(248, 114, 0, 0),   // #f87200
    ];
    let pixels_layer_one: PixelData = PixelData {
        width: 3,
        height: 3,
        pixels: pixels,
    };
    let layer_one = Layer {
        width: 3,
        height: 3,
        kind: LayerColorType {
            kind: LayerColorValue::Rgb,
            alpha: true,
        },
        name: "Background".to_string(),
        pixels: pixels_layer_one,
        properties: vec![],
    };
    layers.push(layer_one);
    xcf.add_layers(&layers);
    let xcf_file = create_file("minimal_nine_pixels.xcf", &mut xcf)?;
    assert_hash(xcf_file.1.to_str().expect(""), "e1748ff2086655bfbcdad61ca4cf27bc7522ab50");
    Ok(())
}

#[test]
fn write_minimal_one_pixel_two_layers() -> Result<(), Error> {
    let mut xcf = XcfCreator::new(11, 1, 1, ColorType::Rgb);
    xcf.add_properties(&vec![]);
    let mut layers = vec![];

    let pixels_layer_one = vec![
        RgbaPixel::new(0, 24, 80, 255),  // #001850
    ];
    let pixels_layer_one: PixelData = PixelData {
        width: 1,
        height: 1,
        pixels: pixels_layer_one,
    };
    let layer_one = Layer {
        width: 1,
        height: 1,
        kind: LayerColorType {
            kind: LayerColorValue::Rgba,
            alpha: true,
        },
        name: "Layer1".to_string(),
        pixels: pixels_layer_one,
        properties: vec![],
    };
    layers.push(layer_one);
    let pixels_layer_two = vec![
        RgbaPixel::new(148, 85, 0, 255),  // #945500
    ];
    let pixels_layer_two: PixelData = PixelData {
        width: 1,
        height: 1,
        pixels: pixels_layer_two,
    };
    let layer_two = Layer {
        width: 1,
        height: 1,
        kind: LayerColorType {
            kind: LayerColorValue::Rgb,
            alpha: true,
        },
        name: "Background".to_string(),
        pixels: pixels_layer_two,
        properties: vec![],
    };
    layers.push(layer_two);
    xcf.add_layers(&layers);
    let xcf_file = create_file("minimal_one_pixel_two_layers", &mut xcf)?;
    assert_hash(xcf_file.1.to_str().expect(""), "04ce4639d6d8168cedd5a6d8067b3babb7e2b432");
    Ok(())
}

/*
#[test]
fn write_miniminiminimal() -> Result<(), Error> {
    let mut minimal_xcf = File::create("tests/samples/create/minimal.xcf")?;
    let mut xcf = XcfCreator::new(11, 4, 4, ColorType::Rgb);
    xcf.add_properties(&vec![]);
    let mut layers = vec![];

    let pixels_layer_one = vec![
        RgbaPixel::new(0, 24, 80, 0),  // #001850
        RgbaPixel::new(115, 43, 0, 0),  // #732b00
        RgbaPixel::new(109, 0, 73, 0),  // #6d0049
        RgbaPixel::new(255, 247, 156, 0),  // #fff79c
    ];
    let pixels_layer_one: PixelData = PixelData {
        width: 2,
        height: 2,
        pixels: pixels_layer_one,
    };
    let layer_one = Layer {
        width: 2,
        height: 2,
        kind: LayerColorType {
            kind: ColorType::Rgb,
            alpha: true,
        },
        name: "Layer1".to_string(),
        pixels: pixels_layer_one,
        properties: vec![],
    };
    layers.push(layer_one);
    let pixels_layer_two = vec![
        RgbaPixel::new(148, 85, 0, 0),  // #945500
        RgbaPixel::new(175, 0, 3, 0),  // #af0003
        RgbaPixel::new(0, 133, 12, 0),  // #00850c
        RgbaPixel::new(255, 255, 255, 0),  // #ffffff

        RgbaPixel::new(229, 247, 16, 0),  // #e5f710
        RgbaPixel::new(255, 255, 255, 0),  // #ffffff
        RgbaPixel::new(55, 37, 230, 0),  // #3725e6
        RgbaPixel::new(132, 230, 37, 0),  // #84e625

        RgbaPixel::new(0, 54, 55, 0),  // #003637
        RgbaPixel::new(55, 0, 54, 0),  // #370036
        RgbaPixel::new(255, 127, 252, 0),  // #ff7ffc
        RgbaPixel::new(117, 107, 97, 0),  // #756b61

        RgbaPixel::new(183, 255, 154, 0),  // #b7ff9a
        RgbaPixel::new(209, 129, 0, 0),  // #d18100
        RgbaPixel::new(209, 0, 75, 0),  // #d1004b
        RgbaPixel::new(134, 159, 216, 0),  // #869fd8
    ];
    let pixels_layer_two: PixelData = PixelData {
        width: 4,
        height: 4,
        pixels: pixels_layer_two,
    };
    let layer_two = Layer {
        width: 4,
        height: 4,
        kind: LayerColorType {
            kind: ColorType::Rgb,
            alpha: true,
        },
        name: "Background".to_string(),
        pixels: pixels_layer_two,
        properties: vec![],
    };
    layers.push(layer_two);
    xcf.add_layers(&layers);
    minimal_xcf.write_all(xcf.data.as_slice())?;
    Ok(())
}
*/
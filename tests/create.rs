use sha1::{Digest, Sha1};
use std::fs::{create_dir, File};
use std::path::{Path, PathBuf};

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

fn create_file(file_name: &'static str, xcf: &mut XcfCreator)-> Result<(File, PathBuf, PathBuf), Error> {
    let dest_dir = Path::new("tests/samples/create");
    let compare_dir = Path::new("tests/samples");

    if let Err(_e) = create_dir(dest_dir) {}

    let path = dest_dir.join(file_name);
    let compare_path = compare_dir.join(file_name);
    let new_file = xcf.save(&path)?;
    Ok((new_file, path, compare_path))
}

#[test]
fn write_minimal_xcf1() -> Result<(), Error> {
    let mut xcf = XcfCreator::new(1, 1, 1, ColorType::Rgb);
    let properties = vec![];
    xcf.add_properties(&properties);
    xcf.add_layers(&vec![]);
    let xcf_file = create_file("minimal_xcf1.xcf", &mut xcf)?;
    let file_hash = "9e54fb4fc2658de528398a66cc684ada35866807";
    assert_hash(xcf_file.1.to_str().expect(""), file_hash);
    assert_hash(xcf_file.2.to_str().expect(""), file_hash);
    Ok(())
}

#[test]
fn write_minimal_xcf3() -> Result<(), Error> {
    let mut xcf = XcfCreator::new(3, 1, 1, ColorType::Rgb);
    let properties = vec![];
    xcf.add_properties(&properties);
    xcf.add_layers(&vec![]);
    let xcf_file = create_file("minimal_xcf3.xcf", &mut xcf)?;
    let file_hash = "1b9d7187a9b783cd3ce16790ab1ebe7a05eac119";
    assert_hash(xcf_file.1.to_str().expect(""), file_hash);
    assert_hash(xcf_file.2.to_str().expect(""), file_hash);
    Ok(())
}

#[test]
fn write_minimal_xcf10() -> Result<(), Error> {
    let mut xcf = XcfCreator::new(10, 1, 1, ColorType::Rgb);
    let properties = vec![];
    xcf.add_properties(&properties);
    xcf.add_layers(&vec![]);
    let xcf_file = create_file("minimal_xcf10.xcf", &mut xcf)?;
    let file_hash = "72dbe0106f48fb25d0fd047acf519f13a3dff086";
    assert_hash(xcf_file.1.to_str().expect(""), file_hash);
    assert_hash(xcf_file.2.to_str().expect(""), file_hash);
    Ok(())
}

#[test]
fn write_minimal_xcf11_without_properties() -> Result<(), Error> {
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
    let file_hash = "6d6e2decc5c6393e83c6ac255e99fdf6617c4a95";
    assert_hash(xcf_file.1.to_str().expect(""), file_hash);
    assert_hash(xcf_file.2.to_str().expect(""), file_hash);
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
    let file_hash = "6d6e2decc5c6393e83c6ac255e99fdf6617c4a95";
    assert_hash(xcf_file.1.to_str().expect(""), file_hash);
    assert_hash(xcf_file.2.to_str().expect(""), file_hash);
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
    let file_hash = "8c4c60c226cd932f4c93dff6ce9ccdc3acc7fbde";
    assert_hash(xcf_file.1.to_str().expect(""), file_hash);
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
    let file_hash = "e1748ff2086655bfbcdad61ca4cf27bc7522ab50";
    assert_hash(xcf_file.1.to_str().expect(""), file_hash);
    assert_hash(xcf_file.2.to_str().expect(""), file_hash);
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
    let xcf_file = create_file("minimal_one_pixel_two_layers.xcf", &mut xcf)?;
    let file_hash = "04ce4639d6d8168cedd5a6d8067b3babb7e2b432";
    assert_hash(xcf_file.1.to_str().expect(""), file_hash);
    assert_hash(xcf_file.2.to_str().expect(""), file_hash);
    Ok(())
}

#[test]
fn write_minimal_9x3() -> Result<(), Error> {
   let mut xcf = XcfCreator::new(11, 9, 3, ColorType::Rgb);
    xcf.add_properties(&vec![]);
    let mut layers = vec![];

    let pixels_layer_two = vec![
        RgbaPixel::new(158, 36, 222, 0),  // #9e24de
        RgbaPixel::new(130, 222, 36, 0),  // #82de24
        RgbaPixel::new(222, 36, 36, 0),   // #de2424
        RgbaPixel::new(158, 36, 222, 0),  // #9e24de
        RgbaPixel::new(130, 222, 36, 0),  // #82de24
        RgbaPixel::new(222, 36, 36, 0),   // #de2424
        RgbaPixel::new(158, 36, 222, 0),  // #9e24de
        RgbaPixel::new(130, 222, 36, 0),  // #82de24
        RgbaPixel::new(222, 36, 36, 0),   // #de2424

        RgbaPixel::new(36, 108, 222, 0),  // #246cde
        RgbaPixel::new(222, 208, 36, 0),  // #ded024
        RgbaPixel::new(5, 97, 48, 0),     // #056130
        RgbaPixel::new(36, 108, 222, 0),  // #246cde
        RgbaPixel::new(222, 208, 36, 0),  // #ded024
        RgbaPixel::new(5, 97, 48, 0),     // #056130
        RgbaPixel::new(36, 108, 222, 0),  // #246cde
        RgbaPixel::new(222, 208, 36, 0),  // #ded024
        RgbaPixel::new(5, 97, 48, 0),     // #056130

        RgbaPixel::new(0, 0, 0, 0),       // #000000
        RgbaPixel::new(136, 231, 219, 0), // #88e7db
        RgbaPixel::new(248, 114, 0, 0),   // #f87200
        RgbaPixel::new(0, 0, 0, 0),       // #000000
        RgbaPixel::new(136, 231, 219, 0), // #88e7db
        RgbaPixel::new(248, 114, 0, 0),   // #f87200
        RgbaPixel::new(0, 0, 0, 0),       // #000000
        RgbaPixel::new(136, 231, 219, 0), // #88e7db
        RgbaPixel::new(248, 114, 0, 0),   // #f87200

    ];
    let pixels_layer_two: PixelData = PixelData {
        width: 9,
        height: 3,
        pixels: pixels_layer_two,
    };
    let layer_two = Layer {
        width: 9,
        height: 3,
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
    let xcf_file = create_file("minimal_9x3_pixels.xcf", &mut xcf)?;
    let file_hash = "b69e3fd8815cffdf722dd440ec5076060e4cde6a";
    assert_hash(xcf_file.1.to_str().expect(""), file_hash);
    //assert_hash(xcf_file.2.to_str().expect(""), file_hash);
    Ok(())
}

#[test]
fn write_minimal_9x9() -> Result<(), Error> {
    let height = 9;
    let mut xcf = XcfCreator::new(11, 9, height, ColorType::Rgb);
    xcf.add_properties(&vec![]);
    let mut layers = vec![];

    let mut pixels_layer_two = vec![];
    for _i in 0..height {
        pixels_layer_two.push(RgbaPixel::new(158, 36, 222, 0));  // #9e24de
        pixels_layer_two.push(RgbaPixel::new(130, 222, 36, 0)); // #82de24
        pixels_layer_two.push(RgbaPixel::new(222, 36, 36, 0)); // #de2424
        pixels_layer_two.push(RgbaPixel::new(36, 108, 222, 0)); // #246cde
        pixels_layer_two.push(RgbaPixel::new(222, 208, 36, 0)); // #ded024
        pixels_layer_two.push(RgbaPixel::new(5, 97, 48, 0)); // #056130
        pixels_layer_two.push(RgbaPixel::new(0, 0, 0, 0)); // #000000
        pixels_layer_two.push(RgbaPixel::new(136, 231, 219, 0)); // #88e7db
        pixels_layer_two.push(RgbaPixel::new(248, 114, 0, 0)); // #f87200
    }
    let pixels_layer_two: PixelData = PixelData {
        width: 9,
        height: height,
        pixels: pixels_layer_two,
    };
    let layer_two = Layer {
        width: 9,
        height: height,
        kind: LayerColorType {
            kind: LayerColorValue::Rgb,
            alpha: false,
        },
        name: "Background".to_string(),
        pixels: pixels_layer_two,
        properties: vec![],
    };
    layers.push(layer_two);
    xcf.add_layers(&layers);
    let xcf_file = create_file("minimal_9x9_pixels.xcf", &mut xcf)?;
    let file_hash = "a1ea8f2e9be410533cbfd81d0dc90835e064767f";
    assert_hash(xcf_file.1.to_str().expect(""), file_hash);
    assert_hash(xcf_file.2.to_str().expect(""), file_hash);
    Ok(())
}

#[test]
fn write_minimal_9x15_diff_bytes() -> Result<(), Error> {
    let height = 15;
    let mut xcf = XcfCreator::new(11, 9, height, ColorType::Rgb);
    xcf.add_properties(&vec![]);
    let mut layers = vec![];

    let mut pixels_layer_two = vec![];
    for _i in 0..height {
        pixels_layer_two.push(RgbaPixel::new(158, 36, 222, 0));  // #9e24de
        pixels_layer_two.push(RgbaPixel::new(130, 222, 36, 0));  // #82de24
        pixels_layer_two.push(RgbaPixel::new(222, 36, 36, 0));  // #de2424
        pixels_layer_two.push(RgbaPixel::new(36, 108, 222, 0));  // #246cde
        pixels_layer_two.push(RgbaPixel::new(222, 208, 36, 0));  // #ded024
        pixels_layer_two.push(RgbaPixel::new(5, 97, 48, 0));  // #056130
        pixels_layer_two.push(RgbaPixel::new(0, 0, 0, 0));  // #000000
        pixels_layer_two.push(RgbaPixel::new(136, 231, 219, 0));  // #88e7db
        pixels_layer_two.push(RgbaPixel::new(248, 114, 0, 0));  // #f87200
    }
    let pixels_layer_two: PixelData = PixelData {
        width: 9,
        height: height,
        pixels: pixels_layer_two,
    };
    let layer_two = Layer {
        width: 9,
        height: height,
        kind: LayerColorType {
            kind: LayerColorValue::Rgb,
            alpha: false,
        },
        name: "Background".to_string(),
        pixels: pixels_layer_two,
        properties: vec![],
    };
    layers.push(layer_two);
    xcf.add_layers(&layers);
    let xcf_file = create_file("minimal_9x15_pixels.xcf", &mut xcf)?;
    let file_hash = "5538a716959ce0b366876e995bc08ae4fc070835";
    assert_hash(xcf_file.1.to_str().expect(""), file_hash);
    assert_hash(xcf_file.2.to_str().expect(""), file_hash);
    Ok(())
}

#[test]
fn write_minimal_9x15_same_bytes() -> Result<(), Error> {
    let width = 9;
    let height = 15;
    let mut xcf = XcfCreator::new(11, width, height, ColorType::Rgb);
    xcf.add_properties(&vec![]);
    let mut layers = vec![];

    let mut pixels_layer_two = vec![];

    for _i in 0..(width * height) {
        pixels_layer_two.push(RgbaPixel::new(54, 201, 84, 0)); // #36c954
    }

    let pixels_layer_two: PixelData = PixelData {
        width: width,
        height: height,
        pixels: pixels_layer_two,
    };
    let layer_two = Layer {
        width: width,
        height: height,
        kind: LayerColorType {
            kind: LayerColorValue::Rgb,
            alpha: false,
        },
        name: "Background".to_string(),
        pixels: pixels_layer_two,
        properties: vec![],
    };
    layers.push(layer_two);
    xcf.add_layers(&layers);
    let xcf_file = create_file("minimal_9x15_same_pixels.xcf", &mut xcf)?;
    let file_hash = "26f5928d28bf68d68c7563885cb42b1174ad1b2a";
    assert_hash(xcf_file.1.to_str().expect(""), file_hash);
    assert_hash(xcf_file.2.to_str().expect(""), file_hash);
    Ok(())
}

#[test]
fn write_minimal_9x65_same_bytes() -> Result<(), Error> {
    let width = 9;
    let height = 65;
    let mut xcf = XcfCreator::new(11, width, height, ColorType::Rgb);
    xcf.add_properties(&vec![]);
    let mut layers = vec![];

    let mut pixels_layer_two = vec![];

    for _i in 0..(width * height) {
        pixels_layer_two.push(RgbaPixel::new(54, 201, 84, 0)); // #36c954
    }

    let pixels_layer_two: PixelData = PixelData {
        width: width,
        height: height,
        pixels: pixels_layer_two,
    };
    let layer_two = Layer {
        width: width,
        height: height,
        kind: LayerColorType {
            kind: LayerColorValue::Rgb,
            alpha: false,
        },
        name: "Background".to_string(),
        pixels: pixels_layer_two,
        properties: vec![],
    };
    layers.push(layer_two);
    xcf.add_layers(&layers);
    let xcf_file = create_file("minimal_9x65_same_pixels.xcf", &mut xcf)?;
    let file_hash = "07f2c58bcc5f33a1bb40b36c307d2adac3f126a3";
    assert_hash(xcf_file.1.to_str().expect(""), file_hash);
    assert_hash(xcf_file.2.to_str().expect(""), file_hash);
    Ok(())
}


#[test]
fn write_minimal_9x138_same_bytes() -> Result<(), Error> {
    let width = 9;
    let height = 138;
    let mut xcf = XcfCreator::new(11, width, height, ColorType::Rgb);
    xcf.add_properties(&vec![]);
    let mut layers = vec![];

    let mut pixels_layer_two = vec![];

    for _i in 0..(width * height) {
        pixels_layer_two.push(RgbaPixel::new(54, 201, 84, 0)); // #36c954
    }

    let pixels_layer_two: PixelData = PixelData {
        width: width,
        height: height,
        pixels: pixels_layer_two,
    };
    let layer_two = Layer {
        width: width,
        height: height,
        kind: LayerColorType {
            kind: LayerColorValue::Rgb,
            alpha: false,
        },
        name: "Background".to_string(),
        pixels: pixels_layer_two,
        properties: vec![],
    };
    layers.push(layer_two);
    xcf.add_layers(&layers);
    let xcf_file = create_file("minimal_9x138_same_pixels.xcf", &mut xcf)?;
    assert_hash(xcf_file.1.to_str().expect(""), "2dae15bf4a97fdf6683de7ab69db0a083d6a320c");
    assert_hash(xcf_file.2.to_str().expect(""), "2dae15bf4a97fdf6683de7ab69db0a083d6a320c");
    Ok(())
}

#[test]
fn write_minimal_138x138_same_bytes() -> Result<(), Error> {
    let width = 138;
    let height = 138;
    let mut xcf = XcfCreator::new(11, width, height, ColorType::Rgb);
    xcf.add_properties(&vec![]);
    let mut layers = vec![];

    let mut pixels_layer_two = vec![];

    for _i in 0..(width * height) {
        pixels_layer_two.push(RgbaPixel::new(54, 201, 84, 0)); // #36c954
    }

    let pixels_layer_two: PixelData = PixelData {
        width: width,
        height: height,
        pixels: pixels_layer_two,
    };
    let layer_two = Layer {
        width: width,
        height: height,
        kind: LayerColorType {
            kind: LayerColorValue::Rgb,
            alpha: false,
        },
        name: "Background".to_string(),
        pixels: pixels_layer_two,
        properties: vec![],
    };
    layers.push(layer_two);
    xcf.add_layers(&layers);
    let xcf_file = create_file("minimal_138x138_same_pixels.xcf", &mut xcf)?;
    let file_hash = "973793f80d32b8505913c3fdddefc803428faae1";
    assert_hash(xcf_file.1.to_str().expect(""), file_hash);
    assert_hash(xcf_file.2.to_str().expect(""), file_hash);
    Ok(())
}

#[test]
fn write_minimal_12x1_diff_bytes() -> Result<(), Error> {
    let width = 12;
    let height = 1;
    let mut xcf = XcfCreator::new(11, width, height, ColorType::Rgb);
    xcf.add_properties(&vec![]);
    let mut layers = vec![];

    let mut pixels_layer_two = vec![];

    for _i in 0..(width * height / 4) {
        pixels_layer_two.push(RgbaPixel::new(0, 0, 0, 0)); //rgb(0, 0, 0)
        pixels_layer_two.push(RgbaPixel::new(54, 201, 84, 0)); // #36c954
        pixels_layer_two.push(RgbaPixel::new(255, 255, 255, 0)); //rgb(255, 255, 255)
        pixels_layer_two.push(RgbaPixel::new(255, 0, 0, 0)); //rgb(255, 0, 0)
    }

    let pixels_layer_two: PixelData = PixelData {
        width: width,
        height: height,
        pixels: pixels_layer_two,
    };
    let layer_two = Layer {
        width: width,
        height: height,
        kind: LayerColorType {
            kind: LayerColorValue::Rgb,
            alpha: false,
        },
        name: "Background".to_string(),
        pixels: pixels_layer_two,
        properties: vec![],
    };
    layers.push(layer_two);
    xcf.add_layers(&layers);
    let xcf_file = create_file("minimal_12x1_diff_pixels.xcf", &mut xcf)?;
    let file_hash = "ce5d0222bbca735fdfb81de03a4fdf1272e0190e";
    assert_hash(xcf_file.1.to_str().expect(""), file_hash);
    assert_hash(xcf_file.2.to_str().expect(""), file_hash);
    Ok(())
}

#[test]
fn write_minimal_12x12_diff_bytes() -> Result<(), Error> {
    let width = 12;
    let height = 12;
    let mut xcf = XcfCreator::new(11, width, height, ColorType::Rgb);
    xcf.add_properties(&vec![]);
    let mut layers = vec![];

    let mut pixels_layer_two = vec![];

    for _i in 0..(width * height / 4) {
        pixels_layer_two.push(RgbaPixel::new(0, 0, 0, 0)); //rgb(0, 0, 0)
        pixels_layer_two.push(RgbaPixel::new(54, 201, 84, 0)); // #36c954
        pixels_layer_two.push(RgbaPixel::new(255, 255, 255, 0)); //rgb(255, 255, 255)
        pixels_layer_two.push(RgbaPixel::new(255, 0, 0, 0)); //rgb(255, 0, 0)
    }

    let pixels_layer_two: PixelData = PixelData {
        width: width,
        height: height,
        pixels: pixels_layer_two,
    };
    let layer_two = Layer {
        width: width,
        height: height,
        kind: LayerColorType {
            kind: LayerColorValue::Rgb,
            alpha: false,
        },
        name: "Background".to_string(),
        pixels: pixels_layer_two,
        properties: vec![],
    };
    layers.push(layer_two);
    xcf.add_layers(&layers);
    let xcf_file = create_file("minimal_12x12_diff_pixels.xcf", &mut xcf)?;
    let file_hash = "b9d98ffc83ebc93d48fba8bb04c7cbedc317470b";
    assert_hash(xcf_file.1.to_str().expect(""), file_hash);
    assert_hash(xcf_file.2.to_str().expect(""), file_hash);
    Ok(())
}

#[test]
fn write_minimal_7x1_diff_bytes() -> Result<(), Error> {
    let width = 7;
    let height = 1;
    let mut xcf = XcfCreator::new(11, width, height, ColorType::Rgb);
    xcf.add_properties(&vec![]);
    let mut layers = vec![];

    let mut pixels_layer_two = vec![];

    pixels_layer_two.push(RgbaPixel::new(0, 0, 0, 0)); //rgb(0, 0, 0)
    pixels_layer_two.push(RgbaPixel::new(0, 0, 0, 0)); //rgb(0, 0, 0)
    pixels_layer_two.push(RgbaPixel::new(0, 0, 0, 0)); //rgb(0, 0, 0)
    pixels_layer_two.push(RgbaPixel::new(54, 201, 84, 0)); // #36c954
    pixels_layer_two.push(RgbaPixel::new(255, 255, 255, 0)); //rgb(255, 255, 255)
    pixels_layer_two.push(RgbaPixel::new(255, 255, 255, 0)); //rgb(255, 255, 255)
    pixels_layer_two.push(RgbaPixel::new(255, 0, 0, 0)); //rgb(255, 0, 0)

    let pixels_layer_two: PixelData = PixelData {
        width: width,
        height: height,
        pixels: pixels_layer_two,
    };
    let layer_two = Layer {
        width: width,
        height: height,
        kind: LayerColorType {
            kind: LayerColorValue::Rgb,
            alpha: false,
        },
        name: "Background".to_string(),
        pixels: pixels_layer_two,
        properties: vec![],
    };
    layers.push(layer_two);
    xcf.add_layers(&layers);
    let xcf_file = create_file("minimal_7x1_diff_pixels.xcf", &mut xcf)?;
    let file_hash = "a2278ac88fafa1a08941f2c742e9f3afd88b2523";
    assert_hash(xcf_file.1.to_str().expect(""), file_hash);
    assert_hash(xcf_file.2.to_str().expect(""), file_hash);
    Ok(())
}

#[test]
fn write_minimal_9x9_diff_bytes() -> Result<(), Error> {
    let width = 9;
    let height = 9;
    let mut xcf = XcfCreator::new(11, width, height, ColorType::Rgb);
    xcf.add_properties(&vec![]);
    let mut layers = vec![];

    let mut pixels_layer_two = vec![];

    for _i in 0..(width * height / 3) {
        let mut v = 255;
        if _i == 0 {
            v = 254;
        }
        pixels_layer_two.push(RgbaPixel::new(v, v, v, 0)); //rgb(255, 255, 255)
        pixels_layer_two.push(RgbaPixel::new(v, v, v, 0)); //rgb(255, 255, 255)
        pixels_layer_two.push(RgbaPixel::new(v, v, v, 0)); //rgb(255, 255, 255)
    }

    let pixels_layer_two: PixelData = PixelData {
        width: width,
        height: height,
        pixels: pixels_layer_two,
    };
    let layer_two = Layer {
        width: width,
        height: height,
        kind: LayerColorType {
            kind: LayerColorValue::Rgb,
            alpha: false,
        },
        name: "Background".to_string(),
        pixels: pixels_layer_two,
        properties: vec![],
    };
    layers.push(layer_two);
    xcf.add_layers(&layers);
    let xcf_file = create_file("minimal_9x9_diff_pixels.xcf", &mut xcf)?;
    let file_hash = "e01c7ae5333cd429b37770af1249626fb3ac19d9";
    assert_hash(xcf_file.1.to_str().expect(""), file_hash);
    assert_hash(xcf_file.2.to_str().expect(""), file_hash);
    Ok(())
}

#[test]
fn write_minimal_36x36_bytes() -> Result<(), Error> {
    let width = 36;
    let height = 36;
    let mut xcf = XcfCreator::new(11, width, height, ColorType::Rgb);
    xcf.add_properties(&vec![]);
    let mut layers = vec![];

    let mut pixels_layer_two = vec![];

    for _i in 0..(width * height / 3) {
        let mut v = 255;
        if _i == 0 || _i == (width * height / 3) - 1 {
            v = 111;
        }
        pixels_layer_two.push(RgbaPixel::new(v, v, v, 0)); //rgb(255, 255, 255)
        pixels_layer_two.push(RgbaPixel::new(v, v, v, 0)); //rgb(255, 255, 255)
        pixels_layer_two.push(RgbaPixel::new(v, v, v, 0)); //rgb(255, 255, 255)
    }

    let pixels_layer_two: PixelData = PixelData {
        width: width,
        height: height,
        pixels: pixels_layer_two,
    };
    let layer_two = Layer {
        width: width,
        height: height,
        kind: LayerColorType {
            kind: LayerColorValue::Rgb,
            alpha: false,
        },
        name: "Background".to_string(),
        pixels: pixels_layer_two,
        properties: vec![],
    };
    layers.push(layer_two);
    xcf.add_layers(&layers);
    let xcf_file = create_file("minimal_36x36_diff_pixels.xcf", &mut xcf)?;
    let file_hash = "cb0d96781444565d9c98b3949dbb8bc83ca7de4a";
    assert_hash(xcf_file.1.to_str().expect(""), file_hash);
    assert_hash(xcf_file.2.to_str().expect(""), file_hash);
    Ok(())
}

#[test]
fn write_minimal_1x32_bytes() -> Result<(), Error> {
    let width = 1;
    let height = 32;
    let mut xcf = XcfCreator::new(11, width, height, ColorType::Rgb);
    xcf.add_properties(&vec![]);
    let mut layers = vec![];

    let mut pixels_layer_one = vec![];

    for _i in 0..16 {
        let v = 255;
        pixels_layer_one.push(RgbaPixel::new(v, v, v, 0)); //rgb(255, 255, 255)
    }
    pixels_layer_one.push(RgbaPixel::new(0, 0, 0, 0)); //rgb(0, 0, 0)
    pixels_layer_one.push(RgbaPixel::new(0, 0, 0, 0)); //rgb(0, 0, 0)

    for _i in 0..14 {
        let v = 255;
        pixels_layer_one.push(RgbaPixel::new(v, v, v, 0)); //rgb(255, 255, 255)
    }

    let pixels_layer_one: PixelData = PixelData {
        width: width,
        height: height,
        pixels: pixels_layer_one,
    };
    let layer_one = Layer {
        width: width,
        height: height,
        kind: LayerColorType {
            kind: LayerColorValue::Rgb,
            alpha: false,
        },
        name: "Background".to_string(),
        pixels: pixels_layer_one,
        properties: vec![],
    };
    layers.push(layer_one);
    xcf.add_layers(&layers);
    let xcf_file = create_file("minimal_1x32_diff_pixels.xcf", &mut xcf)?;
    let file_hash = "f6f4fff136d7e28f108d2135e97cbc059540d9e8";
    assert_hash(xcf_file.1.to_str().expect(""), file_hash);
    assert_hash(xcf_file.2.to_str().expect(""), file_hash);
    Ok(())
}

#[test]
fn write_minimal_1x64_bytes() -> Result<(), Error> {
    let width = 1;
    let height = 64;
    let mut xcf = XcfCreator::new(11, width, height, ColorType::Rgb);
    xcf.add_properties(&vec![]);
    let mut layers = vec![];

    let mut pixels_layer_one = vec![];

    for _i in 0..32 {
        let v = 255;
        pixels_layer_one.push(RgbaPixel::new(v, v, v, 0)); //rgb(255, 255, 255)
    }

    for _i in 0..32 {
        let v = 0;
        pixels_layer_one.push(RgbaPixel::new(v, v, v, 0)); //rgb(0, 0, 0)
    }

    let pixels_layer_one: PixelData = PixelData {
        width: width,
        height: height,
        pixels: pixels_layer_one,
    };
    let layer_one = Layer {
        width: width,
        height: height,
        kind: LayerColorType {
            kind: LayerColorValue::Rgb,
            alpha: false,
        },
        name: "Background".to_string(),
        pixels: pixels_layer_one,
        properties: vec![],
    };
    layers.push(layer_one);
    xcf.add_layers(&layers);
    let xcf_file = create_file("minimal_1x64_diff_pixels.xcf", &mut xcf)?;
    let file_hash = "8ce02afab48ca431ef6e6632a919e1fbd9f1be35";
    assert_hash(xcf_file.1.to_str().expect(""), file_hash);
    assert_hash(xcf_file.2.to_str().expect(""), file_hash);
    Ok(())
}

fn create_layer(width: u32, height: u32, pixel_data: Vec<RgbaPixel>) -> Layer {
    Layer {
        width: width,
        height: height,
        kind: LayerColorType {
            kind: LayerColorValue::Rgb,
            alpha: false,
        },
        name: "Background".to_string(),
        pixels: PixelData {
            width: width,
            height: height,
            pixels: pixel_data,
        },
        properties: vec![],
    }
}

#[test]
fn write_minimal_128x129_diff_bytes() -> Result<(), Error> {
    let width = 128;
    let height = 129;
    let mut xcf = XcfCreator::new(11, width, height, ColorType::Rgb);
    xcf.add_properties(&vec![]);
    let mut layers = vec![];

    let mut pixels_layer_one = vec![];

    for _i in 0..9000 {
        pixels_layer_one.push(RgbaPixel::new(255, 255, 255, 0)); //rgb(255, 255, 255)
    }
    pixels_layer_one.push(RgbaPixel::new(0, 0,0, 0)); //rgb(0, 0, 0)
    pixels_layer_one.push(RgbaPixel::new(0, 0,0, 0)); //rgb(0, 0, 0)

    for _i in 0..7510 {
        pixels_layer_one.push(RgbaPixel::new(255, 255, 255, 0)); //rgb(255, 255, 255)
    }

    let layer_one = create_layer(width, height, pixels_layer_one);
    layers.push(layer_one);
    xcf.add_layers(&layers);
    let xcf_file = create_file("minimal_128x129_diff_pixels.xcf", &mut xcf)?;
    let file_hash = "a4e992bb033a35d892a2402316e4c306197cd347";
    assert_hash(xcf_file.1.to_str().expect(""), file_hash);
    assert_hash(xcf_file.2.to_str().expect(""), file_hash);
    Ok(())
}

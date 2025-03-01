use std::fs::File;
use std::io::Write;

use xcf::data::{property::PropertyIdentifier, rgba::RgbaPixel, color::ColorType, error::Error, xcf::Xcf};
use xcf::create::XcfCreator;

#[test]
fn read_1x1_violet_legacy() -> Result<(), Error> {
    let raw_image = Xcf::open("tests/samples/1x1-violet-legacy.xcf")?;

    assert_eq!(raw_image.header.version.num(), 0);
    assert_eq!(raw_image.dimensions(), (1, 1));

    Ok(())
}

#[test]
fn read_1x1_violet_with_comment() -> Result<(), Error> {
    let raw_image = Xcf::open("tests/samples/1x1-violet-with-comment.xcf")?;

    assert_eq!(raw_image.header.version.num(), 11);
    assert_eq!(raw_image.dimensions(), (1, 1));

    Ok(())
}

#[test]
fn read_245x6734_odd_size_odd_layer() -> Result<(), Error> {
    let raw_image = Xcf::open("tests/samples/246x6734-odd-size-odd-layer.xcf")?;

    assert_eq!(raw_image.header.version.num(), 11);

    assert_eq!(raw_image.dimensions(), (246, 6734));

    assert!(raw_image.layer("Background").is_some());
    assert!(raw_image.layer("Layer 2").is_some());
    assert!(raw_image.layer("Layer 3").is_none());

    assert_eq!(raw_image.layers[1].name, "Background");
    assert_eq!(raw_image.layers[1].dimensions(), (246, 6734));

    assert_eq!(raw_image.layers[0].name, "Layer 2");
    // TODO: check layer offset
    assert_eq!(raw_image.layers[0].dimensions(), (200, 200));

    Ok(())
}

#[test]
fn read_512x512_base_with_alpha() -> Result<(), Error> {
    let raw_image = Xcf::open("tests/samples/512x512-base-with-alpha.xcf")?;

    assert_eq!(raw_image.header.version.num(), 11);
    assert_eq!(raw_image.dimensions(), (512, 512));
    assert_eq!(
        raw_image.layers[2].pixel(0, 0).unwrap().0,
        [215, 194, 78, 255]
    );
    assert_eq!(
        raw_image.layers[2].pixel(1, 0).unwrap().0,
        [215, 194, 78, 128]
    ); // TODO: could be an OBOE

    // TODO: check has alpha

    Ok(())
}

#[test]
fn read_512x512_yellow_base_cloud_layer_empty_layer() -> Result<(), Error> {
    let raw_image = Xcf::open("tests/samples/512x512-yellow-base-cloud-layer-empty-layer.xcf")?;

    assert_eq!(raw_image.header.version.num(), 11);
    assert_eq!(raw_image.dimensions(), (512, 512));
    assert_eq!(raw_image.layers.len(), 3);

    for layer in &raw_image.layers {
        assert_eq!(layer.dimensions(), raw_image.dimensions());
    }

    Ok(())
}

#[test]
fn mini() -> Result<(), Error> {
    let raw_image = Xcf::open("tests/samples/mini.xcf")?;

    assert_eq!(raw_image.header.version.num(), 12);
    assert_eq!(raw_image.dimensions(), (1, 1));
    assert_eq!(raw_image.layers.len(), 1);

    Ok(())
}

#[test]
fn write_minimal() -> Result<(), Error> {
    let mut minimal_xcf = File::create("tests/samples/minimal.xcf")?;
    let mut xcf = XcfCreator::new(1, 1, 1, ColorType::Rgb);
    xcf.add_properties();
    xcf.add_layers();
    minimal_xcf.write_all(xcf.data.as_slice())?;
    Ok(())
}

#[test]
fn write_xcf() -> Result<(), Error> {
    use byteorder::{BigEndian, ByteOrder};

    fn create_signature(gimp_version: u32, data: &mut Vec<u8>, index: &mut u32) {
        let mut signature = format!("gimp xcf v{:03}\0", gimp_version);
        if gimp_version == 1 {
            signature = format!("gimp xcf file\0");
        }
        data.extend_from_slice(signature.as_bytes());
        *index += 14;
    }

    fn extend_u32(value: u32, data: &mut Vec<u8>, index: &mut u32) {
        let mut width_buf = vec![0; 4];
        BigEndian::write_u32(&mut width_buf, value);
        data.extend_from_slice(&width_buf);
        *index += 4;
    }

    fn prop_end(data: &mut Vec<u8>, index: &mut u32) {
        extend_u32(0, data, index); // prop : End
        extend_u32(0, data, index); // size : 0
    }

    // Gimp_String is describe here : https://testing.developer.gimp.org/core/standards/xcf/#basic-data-types
    fn gimp_string(str: &[u8], data: &mut Vec<u8>, index: &mut u32) {
        let str_count = str.iter().count() as u32;
        extend_u32(str_count + 4, data, index);
        data.extend_from_slice(str);
        *index += str_count;
        extend_u32(0, data, index);
    }

    let mut file = File::create("tests/samples/try.xcf")?;
    let mut data = vec!();
    let mut index = 0;
    create_signature(1, &mut data, &mut index);

    extend_u32(1, &mut data, &mut index); // width = 1
    extend_u32(1, &mut data, &mut index); // height = 1
    extend_u32(ColorType::Rgb as u32, &mut data, &mut index); // 0 = RGB TODO : créer un enum

    extend_u32(PropertyIdentifier::PropCompression as u32, &mut data, &mut index); // prop : Compression
    extend_u32(1, &mut data, &mut index); // size compression prop
    //data.extend_from_slice(&[1]); // compression value = RLE
    data.extend_from_slice(&[0]); // compression value = None
    index += 1;

    prop_end(&mut data, &mut index);
    println!("sample : {:?}", index);

    let mut intermediate_buf = vec!();
    extend_u32(0, &mut intermediate_buf, &mut index); // layer_offset[n] : 0 = end
    extend_u32(0, &mut intermediate_buf, &mut index); // channel_offset[] = 0 => end

    let mut layer_one_offset_buf = vec!();
    extend_u32(index + 4, &mut layer_one_offset_buf, &mut index); // layer_offset[0] = le pointer du calque
    layer_one_offset_buf.extend_from_slice(&intermediate_buf);
    data.extend_from_slice(&layer_one_offset_buf);

    extend_u32(1, &mut data, &mut index); // layer[0] : width=1
    extend_u32(1, &mut data, &mut index); // layer[0] : height=1
    extend_u32(0, &mut data, &mut index); // layer[0] : type=RGB

    // layer name :
    gimp_string(b"Background", &mut data, &mut index);

    extend_u32(PropertyIdentifier::PropActiveLayer as u32, &mut data, &mut index); // prop = 2 : active layer
    extend_u32(0, &mut data, &mut index);

    extend_u32(PropertyIdentifier::PropOpacity as u32, &mut data, &mut index); // prop : opacity
    extend_u32(4, &mut data, &mut index); // prop opacity size
    extend_u32(RgbaPixel::new(0, 0, 0, 255).to_u32(), &mut data, &mut index); // color of opacity = black

    extend_u32(PropertyIdentifier::PropMode as u32, &mut data, &mut index); // prop : Mode
    extend_u32(4, &mut data, &mut index); // prop mode size
    extend_u32(0, &mut data, &mut index); // prop mode=normal

    // TODO : à améliorer, ça doit être une valeur en float
    extend_u32(PropertyIdentifier::PropFloatOpacity as u32, &mut data, &mut index); // prop : float opacity
    extend_u32(4, &mut data, &mut index); // prop float opacity size
    let float_slice = [63, 128, 0, 0];
    data.extend_from_slice(&float_slice); // prop float opacity value
    index += 4;

    extend_u32(PropertyIdentifier::PropVisible as u32, &mut data, &mut index); // prop : visible
    extend_u32(4, &mut data, &mut index); // prop visible size
    let float_slice = [0, 0, 0, 1];
    data.extend_from_slice(&float_slice); // prop visible value
    index += 4;

    extend_u32(PropertyIdentifier::PropLinked as u32, &mut data, &mut index); // prop : linked
    extend_u32(4, &mut data, &mut index); // prop linked size
    extend_u32(0, &mut data, &mut index); // prop linked value

    //PropColorTag

    prop_end(&mut data, &mut index);

    // hierarchy
    extend_u32(index + 8, &mut data, &mut index); // hierarchy offset.
    extend_u32(0, &mut data, &mut index); // mask offset

    // https://testing.developer.gimp.org/core/standards/xcf/#the-hierarchy-structure
    extend_u32(1, &mut data, &mut index); // width=1
    extend_u32(1, &mut data, &mut index); // height=1
    extend_u32(3, &mut data, &mut index); // bpp=3 : RGB color without alpha in 8-bit precision
    extend_u32(index + 8, &mut data, &mut index); // offset[0]
    extend_u32(0, &mut data, &mut index); // offset[n] = 0 => end

    extend_u32(1, &mut data, &mut index); // level[0] width =1
    extend_u32(1, &mut data, &mut index); // level[0] height =1
    extend_u32(index + 8, &mut data, &mut index); // offset= le pointer du contenu
    extend_u32(0, &mut data, &mut index); // data_offset[0] = 0 => end

    //let slice = [00, 158, 00, 36, 00, 222]; // violet r: 158, g: 23, b: 222  with RLE compression
    let slice = [158, 36, 222]; // violet r: 158, g: 23, b: 222  without compression
    data.extend_from_slice(&slice);

    /*
    for d in &data {
        print!("{:02x} ", d);
    }
    */
    file.write_all(data.as_slice())?;
    Ok(())
}

#[test]
#[ignore]
fn read_1024x1024_better_compression() -> Result<(), Error> {
    let raw_image = Xcf::open("tests/samples/1024x1024-better-compression.xcf")?;

    assert_eq!(raw_image.header.version.num(), 11);
    assert_eq!(raw_image.dimensions(), (1024, 1024));
    // TODO: check bg does not have alpha
    assert_eq!(
        raw_image
            .layer("Background")
            .unwrap()
            .pixels
            .pixel(220, 203)
            .unwrap()
            .0,
        [125, 125, 125, 255], // TODO: the alpha in this test is wrong - is it 125?
    );

    assert_eq!(
        raw_image
            .layer("Layer 1")
            .unwrap()
            .pixels
            .pixel(220, 203)
            .unwrap()
            .0,
        [215, 194, 78, 255]
    );

    Ok(())
}

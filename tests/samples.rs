use xcf::{Error, Xcf};

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
fn write_xcf() -> Result<(), Error> {
    use std::fs::File;
    use std::io::Write;
    use byteorder::{BigEndian, ByteOrder};

    fn extend_u32(value: u32, data: &mut Vec<u8>) {
        let mut width_buf = vec![0; 4];
        BigEndian::write_u32(&mut width_buf, value);
        data.extend_from_slice(&width_buf);
    }

    let mut file = File::create("tests/samples/try.xcf")?;
    let mut data = "gimp xcf v011\0".as_bytes().to_vec();

    extend_u32(1, &mut data); // width = 1
    extend_u32(1, &mut data); // height = 1
    extend_u32(0, &mut data); // 0 = RGB TODO : créer un enum

    extend_u32(17, &mut data); // prop : Compression
    extend_u32(1, &mut data); // size compression prop
    data.extend_from_slice(&[0]); // compression value = None

    extend_u32(0, &mut data); // prop : End
    extend_u32(0, &mut data); // size : 0

    extend_u32(0, &mut data); // layer_offset[] = 0 => fin
    extend_u32(0, &mut data); // channel_offset[] = 0 => fin

    for d in &data {
        print!("{:02x} ", d);
    }
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

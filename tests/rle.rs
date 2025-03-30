use xcf::create::rle_compress;

#[test]
fn rle_compression_four_pixels() {
    assert_eq!(
        rle_compress(&vec!(222, 36, 36, 222)),
        vec!(252, 222, 36, 36, 222)
    );
    assert_eq!(
        rle_compress(&vec!(158, 0, 255, 43)),
        vec!(252, 158, 0, 255, 43)
    );
    assert_eq!(
        rle_compress(&vec!(0, 158, 5, 34)),
        vec!(252, 0, 158, 5, 34)
    );
    assert_eq!(
        rle_compress(&vec!(0, 0, 114, 121)),
        vec!(1, 0, 254, 114, 121) // 01 00 fe 72 79
    );
}

#[test]
fn rle_compression_nine_pixels() {
    assert_eq!(
        rle_compress(&vec!(222, 36, 36, 222, 36, 48, 0, 219, 0)),
        vec!(247, 222, 36, 36, 222, 36, 48, 0, 219, 0)
    );
}

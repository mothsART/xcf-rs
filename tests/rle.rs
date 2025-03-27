use xcf::create::rle_compress;

#[test]
fn four_pixels() {
    assert_eq!(
        rle_compress(&vec!(158, 0, 255, 43)),
        vec!(252, 158, 0, 255, 43)
    );
    assert_eq!(
        rle_compress(&vec!(0, 0, 114, 121)),
        vec!(1, 0, 254, 0, 121)
    );
    assert_eq!(
        rle_compress(&vec!(0, 158, 5, 34)),
        vec!(252, 0, 158, 5, 34)
    );
}
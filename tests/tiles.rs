use xcf_rs::data::layer::Layer;
use xcf_rs::data::tiles::Tiles;
use xcf_rs::{LayerColorValue, LayerColorType};
use xcf_rs::data::pixeldata::PixelData;

fn create_fake_layer(width: u32, height: u32) -> Layer {
    Layer {
        width: width,
        height: height,
        kind: LayerColorType {
            kind: LayerColorValue::Rgb,
            alpha: true,
        },
        name: "background".to_string(),
        properties: vec![],
        pixels: PixelData {
            width: width,
            height: height,
            pixels: vec![]
        }
    }
}

#[test]
fn tiles_count_10x10() {
    let layer = create_fake_layer(10, 10);
    let tiles = Tiles::new(&layer);
    assert_eq!(tiles.nb, 1);
}

#[test]
fn tiles_count_64x64() {
    let layer = create_fake_layer(64, 64);
    let tiles = Tiles::new(&layer);
    assert_eq!(tiles.nb, 1);
}

#[test]
fn tiles_count_65x64() {
    let layer = create_fake_layer(65, 64);
    let tiles = Tiles::new(&layer);
    assert_eq!(tiles.nb, 2);
}

#[test]
fn tiles_count_65x65() {
    let layer = create_fake_layer(65, 65);
    let tiles = Tiles::new(&layer);
    assert_eq!(tiles.nb, 4);
}

#[test]
fn tiles_count_128x128() {
    let layer = create_fake_layer(128, 128);
    let tiles = Tiles::new(&layer);
    assert_eq!(tiles.nb, 4);
}

#[test]
fn tiles_count_128x129() {
    let layer = create_fake_layer(128, 129);
    let tiles = Tiles::new(&layer);
    assert_eq!(tiles.nb, 6);
}
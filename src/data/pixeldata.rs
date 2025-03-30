use crate::RgbaPixel;

// TODO: Make this an enum? We should store a buffer that matches the channels present.
#[derive(Clone, Debug, PartialEq)]
pub struct PixelData {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<RgbaPixel>,
}

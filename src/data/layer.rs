use crate::{LayerColorType, PixelData, Property};

#[derive(Debug, PartialEq)]
pub struct Layer {
    pub width: u32,
    pub height: u32,
    pub kind: LayerColorType,
    pub name: String,
    pub properties: Vec<Property>,
    pub pixels: PixelData,
}

use crate::{ColorType, Precision, Property, Version};

#[derive(Debug, PartialEq)]
pub struct XcfHeader {
    pub version: Version,
    pub width: u32,
    pub height: u32,
    pub color_type: ColorType,
    pub precision: Precision,
    pub properties: Vec<Property>,
}

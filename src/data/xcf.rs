use crate::{Layer, XcfHeader};

#[derive(Debug, PartialEq, Clone)]
pub enum XcfCompression {
    None = 0,
    Rle = 1,
    Zlib = 2,
    Fractal = 3,
}

impl XcfCompression {
    pub fn to_u8(&self) -> u8 {
        self.clone() as u8
    }
}

#[derive(Debug)]
pub struct Xcf {
    pub header: XcfHeader,
    /// List of layers in the XCF file, in the order they are stored in the file.
    /// (I believe this is top layer to bottom layer)
    ///
    /// See [`Xcf::layer`](Xcf::layer) to get a layer by name.
    pub layers: Vec<Layer>,
}

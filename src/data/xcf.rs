use crate::{XcfHeader, Layer};

#[derive(Debug)]
pub struct Xcf {
    pub header: XcfHeader,
    /// List of layers in the XCF file, in the order they are stored in the file.
    /// (I believe this is top layer to bottom layer)
    ///
    /// See [`Xcf::layer`](Xcf::layer) to get a layer by name.
    pub layers: Vec<Layer>,
}

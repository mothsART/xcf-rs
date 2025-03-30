//! Read pixel data from GIMP's native XCF files.
//!
//! See [`Xcf`] for usage (methods `open` and `load`). For extracting pixel data, you probably want
//! to access a layer via `Xcf::layer` and `Layer::raw_sub_buffer`, which you can use to
//! create `ImageBuffer`s from the `image` crate. You can also do direct pixel access via
//! `Layer::pixel`.
//!
//! [`Xcf`]: struct.Xcf.html

#[macro_use]
extern crate derive_error;
extern crate byteorder;

use std::io::{Read, Seek, SeekFrom};

use byteorder::{BigEndian, ReadBytesExt};
use std::borrow::Cow;
use std::cmp;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub mod create;
pub mod data;
pub mod parser;

use crate::data::{
    color::ColorType, error::Error, header::XcfHeader, layer::Layer, pixeldata::PixelData,
    precision::Precision, property::PropertyPayload, rgba::RgbaPixel, version::Version, xcf::Xcf,
};
use crate::parser::ParseVersion;

use crate::data::property::{Property, PropertyIdentifier};

impl Precision {
    fn parse<R: Read>(mut rdr: R, version: Version) -> Result<Self, Error> {
        let precision = rdr.read_u32::<BigEndian>()?;
        Ok(match version.num() {
            4 => match precision {
                0 => Precision::NonLinearU8,
                1 => Precision::NonLinearU16,
                2 => Precision::LinearU32,
                3 => Precision::LinearF16,
                4 => Precision::LinearF32,
                _ => return Err(Error::InvalidPrecision),
            },
            5..=6 => match precision {
                100 => Precision::LinearU8,
                150 => Precision::NonLinearU8,
                200 => Precision::LinearU16,
                250 => Precision::NonLinearU16,
                300 => Precision::LinearU32,
                350 => Precision::NonLinearU32,
                400 => Precision::LinearF16,
                450 => Precision::NonLinearF16,
                500 => Precision::LinearF32,
                550 => Precision::NonLinearF32,
                _ => return Err(Error::InvalidPrecision),
            },
            7.. => match precision {
                100 => Precision::LinearU8,
                150 => Precision::NonLinearU8,
                175 => Precision::PerceptualU8,
                200 => Precision::LinearU16,
                250 => Precision::NonLinearU16,
                275 => Precision::PerceptualU16,
                300 => Precision::LinearU32,
                350 => Precision::NonLinearU32,
                375 => Precision::PerceptualU32,
                500 => Precision::LinearF16,
                550 => Precision::NonLinearF16,
                575 => Precision::PerceptualF16,
                600 => Precision::LinearF32,
                650 => Precision::NonLinearF32,
                675 => Precision::PerceptualF32,
                700 => Precision::LinearF64,
                750 => Precision::NonLinearF64,
                775 => Precision::PerceptualF64,
                _ => return Err(Error::InvalidPrecision),
            },
            _ => return Err(Error::InvalidPrecision),
        })
    }
}

impl PropertyPayload {
    fn parse<R: Read>(
        mut rdr: R,
        kind: PropertyIdentifier,
        length: usize,
    ) -> Result<PropertyPayload, Error> {
        use self::PropertyIdentifier::*;
        Ok(match kind {
            PropEnd => PropertyPayload::End,
            _ => {
                let mut p = vec![0; length];
                rdr.read_exact(&mut p)?;
                PropertyPayload::Unknown(p)
            }
        })
    }
}

impl Layer {
    fn parse<R: Read + Seek>(mut rdr: R, version: Version) -> Result<Layer, Error> {
        let width = rdr.read_u32::<BigEndian>()?;
        let height = rdr.read_u32::<BigEndian>()?;
        let kind = LayerColorType::new(rdr.read_u32::<BigEndian>()?)?;
        let name = read_gimp_string(&mut rdr)?;
        let properties = Property::parse_list(&mut rdr)?;
        let hptr = rdr.read_uint::<BigEndian>(version.bytes_per_offset())?;
        let current_pos = rdr.stream_position()?;
        rdr.seek(SeekFrom::Start(hptr))?;
        let pixels = PixelData::parse_hierarchy(&mut rdr, version)?;
        rdr.seek(SeekFrom::Start(current_pos))?;
        // TODO
        // let mptr = rdr.read_uint::<BigEndian>(version.bytes_per_offset())?;
        Ok(Layer {
            width,
            height,
            kind,
            name,
            properties,
            pixels,
        })
    }

    pub fn pixel(&self, x: u32, y: u32) -> Option<RgbaPixel> {
        self.pixels.pixel(x, y)
    }

    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn raw_rgba_buffer(&self) -> Cow<[RgbaPixel]> {
        Cow::from(&self.pixels.pixels)
    }

    pub fn raw_sub_rgba_buffer(&self, x: u32, y: u32, width: u32, height: u32) -> Vec<u8> {
        self.pixels.raw_sub_rgba_buffer(x, y, width, height)
    }
}

#[derive(Debug, PartialEq)]
pub struct LayerColorType {
    pub kind: ColorType,
    pub alpha: bool,
}

impl LayerColorType {
    fn new(identifier: u32) -> Result<LayerColorType, Error> {
        let kind = ColorType::new(identifier / 2)?;
        let alpha = identifier % 2 == 1;
        Ok(LayerColorType { alpha, kind })
    }
}

pub struct TileCursor {
    width: u32,
    height: u32,
    channels: u32,
    x: u32,
    y: u32,
    i: u32,
}

// TODO: I like the use of a struct but this isn't really any kind of cursor.
// The use of a struct allows us to seperate the state we need to refer to from the number of
// stuff we need to store within the "algorithm." A better design is very welcome! i should be
// moved into feed as a local.
impl TileCursor {
    fn new(width: u32, height: u32, tx: u32, ty: u32, channels: u32) -> TileCursor {
        TileCursor {
            width,
            height,
            channels,
            x: tx * 64,
            y: ty * 64,
            i: 0,
        }
    }

    /// Feed the cursor a stream starting at the beginning of an XCF tile structure.
    fn feed<R: Read>(&mut self, mut rdr: R, pixels: &mut [RgbaPixel]) -> Result<(), Error> {
        let twidth = cmp::min(self.x + 64, self.width) - self.x;
        let theight = cmp::min(self.y + 64, self.height) - self.y;
        let base_offset = self.y * self.width + self.x;
        // each channel is laid out one after the other
        let mut channel = 0;
        while channel < self.channels {
            while self.i < twidth * theight {
                let determinant = rdr.read_u8()?;
                if determinant < 127 {
                    // A short run of identical bytes
                    let run = u32::from(determinant + 1);
                    let v = rdr.read_u8()?;
                    for i in (self.i)..(self.i + run) {
                        let index = base_offset + (i / twidth) * self.width + i % twidth;
                        pixels[index as usize].0[channel as usize] = v;
                    }
                    self.i += run;
                } else if determinant == 127 {
                    // A long run of identical bytes
                    let run = u32::from(rdr.read_u16::<BigEndian>()?);
                    let v = rdr.read_u8()?;
                    for i in (self.i)..(self.i + run) {
                        let index = base_offset + (i / twidth) * self.width + i % twidth;
                        pixels[index as usize].0[channel as usize] = v;
                    }
                    self.i += run;
                } else if determinant == 128 {
                    // A long run of different bytes
                    let stream_run = u32::from(rdr.read_u16::<BigEndian>()?);
                    for i in (self.i)..(self.i + stream_run) {
                        let index = base_offset + (i / twidth) * self.width + i % twidth;
                        let v = rdr.read_u8()?;
                        pixels[index as usize].0[channel as usize] = v;
                    }
                    self.i += stream_run;
                } else {
                    // A short run of different bytes
                    let stream_run = 256 - u32::from(determinant);
                    for i in (self.i)..(self.i + stream_run) {
                        let index = base_offset + (i / twidth) * self.width + i % twidth;
                        let v = rdr.read_u8()?;
                        pixels[index as usize].0[channel as usize] = v;
                    }
                    self.i += stream_run;
                }
            }

            self.i = 0;
            channel += 1;
        }
        Ok(())
    }
}

fn read_gimp_string<R: Read>(mut rdr: R) -> Result<String, Error> {
    let length = rdr.read_u32::<BigEndian>()?;
    let mut buffer = vec![0; length as usize - 1];
    rdr.read_exact(&mut buffer)?;
    // read the DUMB trailing null byte... uhh GIMP team RIIR already? ;p
    rdr.read_exact(&mut [0u8])?;
    Ok(String::from_utf8(buffer)?)
}

/// A GIMP XCF file.
///
/// If you need to access multiple layers at once, access layers field and use `split_at`.
impl Xcf {
    /// Open an XCF file at the path specified.
    pub fn open<P: AsRef<Path>>(p: P) -> Result<Xcf, Error> {
        let rdr = BufReader::new(File::open(p)?);
        Xcf::load(rdr)
    }

    /// Read an XCF file from a Reader.
    pub fn load<R: Read + Seek>(mut rdr: R) -> Result<Xcf, Error> {
        let header = XcfHeader::parse(&mut rdr)?;

        let mut layers = Vec::new();
        loop {
            let layer_pointer = rdr.read_uint::<BigEndian>(header.version.bytes_per_offset())?;
            if layer_pointer == 0 {
                break;
            }
            let current_pos = rdr.stream_position()?;
            rdr.seek(SeekFrom::Start(layer_pointer))?;
            layers.push(Layer::parse(&mut rdr, header.version)?);
            rdr.seek(SeekFrom::Start(current_pos))?;
        }

        // TODO: Read channels

        Ok(Xcf { header, layers })
    }

    /// Get the width of the canvas.
    pub fn width(&self) -> u32 {
        self.header.width
    }

    /// Get the height of the canvas.
    pub fn height(&self) -> u32 {
        self.header.height
    }

    // Get the dimensions (width, height) of the canvas.
    pub fn dimensions(&self) -> (u32, u32) {
        (self.width(), self.height())
    }

    /// Get a reference to a layer by `name`.
    pub fn layer(&self, name: &str) -> Option<&Layer> {
        self.layers.iter().find(|l| l.name == name)
    }

    /// Get a mutable reference to a layer by `name`.
    pub fn layer_mut(&mut self, name: &str) -> Option<&mut Layer> {
        self.layers.iter_mut().find(|l| l.name == name)
    }
}

impl XcfHeader {
    fn parse<R: Read>(mut rdr: R) -> Result<XcfHeader, Error> {
        let mut magic = [0u8; 9];
        rdr.read_exact(&mut magic)?;
        if magic != *b"gimp xcf " {
            return Err(Error::InvalidFormat);
        }

        let version = Version::parse(&mut rdr)?;
        /*if version.num() > 11 {
            return Err(Error::UnknownVersion);
        }*/

        rdr.read_exact(&mut [0u8])?;

        let width = rdr.read_u32::<BigEndian>()?;
        let height = rdr.read_u32::<BigEndian>()?;

        let color_type = ColorType::new(rdr.read_u32::<BigEndian>()?)?;

        /*
        if color_type != ColorType::Rgb {
            unimplemented!("Only RGB/RGBA color images supported");
        }
        */

        let precision = if version.num() >= 4 {
            Precision::parse(&mut rdr, version)?
        } else {
            Precision::NonLinearU8
        };

        let properties = Property::parse_list(&mut rdr)?;

        Ok(XcfHeader {
            version,
            width,
            height,
            color_type,
            precision,
            properties,
        })
    }
}

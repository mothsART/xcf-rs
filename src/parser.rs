use std::io::{Read, Seek, SeekFrom};

use byteorder::{BigEndian, ReadBytesExt};

use crate::data::error::Error;
use crate::data::pixeldata::PixelData;
use crate::data::rgba::RgbaPixel;
use crate::data::version::Version;
use crate::TileCursor;

pub trait ParseVersion {
    fn parse<R: Read + std::fmt::Debug>(rdr: R) -> Result<Self, Error>
    where
        Self: Sized;
}

impl ParseVersion for Version {
    fn parse<R: Read + std::fmt::Debug>(mut rdr: R) -> Result<Self, Error> {
        let mut v = [0; 4];
        rdr.read_exact(&mut v)?;
        match &v {
            b"file" => Ok(Self(0)),
            [b'v', ver @ ..] => Ok(Self(
                String::from_utf8_lossy(ver)
                    .parse()
                    .map_err(|_| Error::UnknownVersion)?,
            )),
            _ => Err(Error::UnknownVersion),
        }
    }
}

impl PixelData {
    /// Parses the (silly?) hierarchy structure in the xcf file into a pixel array
    /// Makes lots of assumptions! Only supports RGBA for now.
    pub fn parse_hierarchy<R: Read + Seek + std::fmt::Debug>(
        mut rdr: R,
        version: Version,
    ) -> Result<PixelData, Error> {
        // read the hierarchy
        let width = rdr.read_u32::<BigEndian>()?;
        let height = rdr.read_u32::<BigEndian>()?;
        let bpp = rdr.read_u32::<BigEndian>()?;
        /*
        if bpp != 3 && bpp != 4 {
            return Err(Error::NotSupported);
        }
        */
        let lptr = rdr.read_uint::<BigEndian>(version.bytes_per_offset())?;
        let _dummpy_ptr_pos = rdr.stream_position()?;
        rdr.seek(SeekFrom::Start(lptr))?;
        // read the level
        let level_width = rdr.read_u32::<BigEndian>()?;
        let level_height = rdr.read_u32::<BigEndian>()?;
        if level_width != width || level_height != height {
            return Err(Error::InvalidFormat);
        }

        let mut pixels = vec![RgbaPixel([0, 0, 0, 255]); (width * height) as usize];
        let mut next_tptr_pos;

        let tiles_x = (f64::from(width) / 64.0).ceil() as u32;
        let tiles_y = (f64::from(height) / 64.0).ceil() as u32;
        for ty in 0..tiles_y {
            for tx in 0..tiles_x {
                let tptr = rdr.read_uint::<BigEndian>(version.bytes_per_offset())?;
                next_tptr_pos = rdr.stream_position()?;
                rdr.seek(SeekFrom::Start(tptr))?;

                let mut cursor = TileCursor::new(width, height, tx, ty, bpp);
                cursor.feed(&mut rdr, &mut pixels)?;

                rdr.seek(SeekFrom::Start(next_tptr_pos))?;
            }
        }

        // rdr.seek(SeekFrom::Start(dummpy_ptr_pos))?;
        // TODO: dummy levels? do we need to consider them?
        // if we do:
        /*loop {
            let dummy_level_ptr = rdr.read_u32::<BigEndian>()?;
            if dummy_level_ptr == 0 {
                break;
            }
        }*/
        // we are now at the end of the heirarchy structure.

        Ok(PixelData {
            pixels,
            width,
            height,
        })
    }

    pub fn pixel(&self, x: u32, y: u32) -> Option<RgbaPixel> {
        if x >= self.width || y >= self.height {
            return None;
        }
        Some(self.pixels[(y * self.width + x) as usize])
    }

    /// Creates a raw sub buffer from self.
    ///
    /// # Panics
    ///
    /// Panics if a pixel access is out of bounds.
    pub fn raw_sub_rgba_buffer(&self, x: u32, y: u32, width: u32, height: u32) -> Vec<u8> {
        let mut sub = Vec::with_capacity((width * height * 4) as usize);
        for _y in y..(y + height) {
            for _x in x..(x + width) {
                if _y > self.height || _x > self.width {
                    panic!("Pixel access is out of bounds");
                }
                sub.extend_from_slice(&self.pixel(_x, _y).unwrap().0);
            }
        }
        sub
    }
}

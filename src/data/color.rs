use crate::Error;

#[repr(u32)]
#[derive(Debug, PartialEq, Clone)]
pub enum ColorType {
    Rgb = 0,
    Grayscale = 1,
    Indexed = 2,
}

impl ColorType {
    pub(crate) fn new(kind: u32) -> Result<ColorType, Error> {
        use self::ColorType::*;
        Ok(match kind {
            0 => Rgb,
            1 => Grayscale,
            2 => Indexed,
            _ => return Err(Error::InvalidFormat),
        })
    }
}
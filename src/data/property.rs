use std::io::Read;
use byteorder::{BigEndian, ReadBytesExt};

use crate::RgbaPixel;
use crate::data::error::Error;
use crate::data::xcf::XcfCompression;

use super::rgba;

#[derive(Debug, PartialEq)]
pub struct ResolutionProperty {
    pub xres: f32,
    pub yres: f32,
}

#[derive(Debug, PartialEq)]
pub struct ParasiteProperty {
    pub name: String,
    pub flags: u32,
    pub data: String,
}

#[derive(Debug, PartialEq)]
pub enum PropertyPayload {
    ColorMap { colors: usize },
    End,
    Compression(XcfCompression),
    ResolutionProperty(ResolutionProperty),
    Tatoo(u32),
    Unit(u32),
    Parasites(Vec<ParasiteProperty>),
    // layer property
    ActiveLayer(),
    OpacityProperty(RgbaPixel),
    FloatOpacityProperty(),
    VisibleProperty(),
    LinkedLayerProperty(u32),
    ColorTagLayerProperty(u32),
    LockContentLayerProperty(u32),
    LockAlphaLayerProperty(u32),
    LockPositionLayerProperty(u32),
    ApplyMaskLayerProperty(u32),
    EditMaskLayerProperty(u32),
    ShowMaskLayerProperty(u32),
    OffsetsLayerProperty(u32, u32),
    ModeLayerProperty(u32),
    BlendSpaceLayerProperty(u32),
    CompositeSpaceLayerProperty(u32),
    CompositeModeLayerProperty(u32),
    Unknown(Vec<u8>),
}

#[derive(Debug, PartialEq)]
pub struct Property {
    pub kind: PropertyIdentifier,
    pub length: usize,
    pub payload: PropertyPayload,
}

impl Property {
    // TODO: GIMP usually calculates sizes based on data and goes from that instead of the reported
    // property length... (for known properties)
    fn guess_size(&self) -> usize {
        match self.payload {
            PropertyPayload::ColorMap { colors, .. } => {
                /* apparently due to a GIMP bug sometimes self.length will be n + 4 */
                3 * colors + 4
            }
            // this is the best we can do otherwise
            _ => self.length,
        }
    }

    fn parse<R: Read>(mut rdr: R) -> Result<Property, Error> {
        let kind = PropertyIdentifier::new(rdr.read_u32::<BigEndian>()?);
        let length = rdr.read_u32::<BigEndian>()? as usize;
        let payload = PropertyPayload::parse(&mut rdr, kind, length)?;
        Ok(Property {
            kind,
            length,
            payload,
        })
    }

    pub fn parse_list<R: Read>(mut rdr: R) -> Result<Vec<Property>, Error> {
        let mut props = Vec::new();
        loop {
            let p = Property::parse(&mut rdr)?;
            if let PropertyIdentifier::PropEnd = p.kind {
                break;
            }
            // only push non end
            props.push(p);
        }
        Ok(props)
    }
}

macro_rules! prop_ident_gen {
    (
        #[derive(Debug, Clone, Copy, PartialEq)]
        #[repr(u32)]
        pub enum PropertyIdentifier {
            //Unknown(u32),
            $(
                $prop:ident = $val:expr
            ),+,
        }
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq)]
        #[repr(u32)]
        pub enum PropertyIdentifier {
            $(
                $prop = $val
            ),+,
            // we have to put this at the end, since otherwise it will try to have value zero,
            // we really don't care what it is as long as it doesn't conflict with anything else
            // (however in the macro we have to put it first since it's a parsing issue)
            //Unknown(u32),
        }

        impl PropertyIdentifier {
            pub fn new(prop: u32) -> PropertyIdentifier {
                match prop {
                    $(
                        $val => PropertyIdentifier::$prop
                    ),+,
                    _ => todo!()
                    //_ => PropertyIdentifier::Unknown(prop),
                }
            }
        }
    }
}

prop_ident_gen! {
    #[derive(Debug, Clone, Copy, PartialEq)]
    #[repr(u32)]
    pub enum PropertyIdentifier {
        PropEnd = 0,
        PropColormap = 1,
        PropActiveLayer = 2,
        PropActiveChannel = 3,
        PropSelection = 4,
        PropFloatingSelection = 5,
        PropOpacity = 6,
        PropMode = 7,
        PropVisible = 8,
        PropLinked = 9,
        PropLockAlpha = 10,
        PropApplyMask = 11,
        PropEditMask = 12,
        PropShowMask = 13,
        PropOffsets = 15,
        PropCompression = 17,
        TypeIdentification = 18,
        PropResolution = 19,
        PropTattoo = 20,
        PropParasites = 21,
        PropUnit = 22,
        PropPaths = 23,
        PropUserUnit = 24,
        PropVectors = 25,
        PropTextLayerFlags = 26,
        PropOldSamplePoints = 27,
        PropLockContent = 28,
        PropLockPosition = 32,
        PropFloatOpacity = 33,
        PropColorTag = 34,
        PropCompositeMode = 35,
        PropCompositeSpace = 36,
        PropBlendSpace = 37,
        PropFloatColor = 38,
        PropSamplePoints = 39,
        PropItemSet = 40,
        PropItemSetItem = 41,
        PropLockVisibility = 42,
        PropSelectedPath = 43,
        PropFilterRegion = 44,
        PropFilterArgument = 45,
        PropFilterClip = 46,
    }
}

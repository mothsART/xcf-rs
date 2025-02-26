#[derive(Debug, PartialEq)]
pub enum PropertyPayload {
    ColorMap { colors: usize },
    End,
    Unknown(Vec<u8>),
}

#[derive(Debug, PartialEq)]
pub struct Property {
    pub kind: PropertyIdentifier,
    pub length: usize,
    pub payload: PropertyPayload,
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

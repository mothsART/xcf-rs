use byteorder::{BigEndian, ByteOrder};

use crate::data::color::ColorType;
use crate::data::property::Property;
use crate::data::property::PropertyPayload;
use crate::PropertyIdentifier;
use crate::RgbaPixel;

pub struct XcfCreator {
    pub version: u16,
    pub data: Vec<u8>,
    pub index: u32,
}

//impl Creator for Xcf {
impl XcfCreator {
    fn extend_u32(&mut self, value: u32) {
        let mut width_buf = vec![0; 4];
        BigEndian::write_u32(&mut width_buf, value);
        self.data.extend_from_slice(&width_buf);
        self.index += 4;
    }

    fn extend_u64(&mut self, value: u64) {
        let mut width_buf = vec![0; 8];
        BigEndian::write_u64(&mut width_buf, value);
        self.data.extend_from_slice(&width_buf);
        self.index += 8;
    }

    fn buf_extend_u32(&mut self, data: &mut Vec<u8>, index: &mut u32, value: u32) {
        let size = 4;
        let mut width_buf = vec![0; size];
        BigEndian::write_u32(&mut width_buf, value);
        data.extend_from_slice(&width_buf);
        *index += size as u32;
    }

    fn buf_extend_u64(&mut self, data: &mut Vec<u8>, index: &mut u32, value: u64) {
        let size = 8;
        let mut width_buf = vec![0; size];
        BigEndian::write_u64(&mut width_buf, value);
        data.extend_from_slice(&width_buf);
        *index += size as u32;
    }

    fn create_signature(&mut self, gimp_version: u16) {
        let mut signature = format!("gimp xcf v{:03}\0", gimp_version);
        if gimp_version == 1 {
            signature = format!("gimp xcf file\0");
        }
        self.data.extend_from_slice(signature.as_bytes());
        self.index += 14;
    }

    fn gimp_string(&mut self, str: &[u8]) {
        let str_count = str.iter().count() as u32;
        self.extend_u32(str_count + 4);
        self.data.extend_from_slice(str);
        self.index += str_count;
        self.extend_u32(0);
    }

    pub fn new(version: u16, width: u32, height: u32, color_type: ColorType) -> Self {
        let data = vec!();
        let index = 0;

        let mut _self = XcfCreator {
            version,
            data,
            index
        };
        _self.create_signature(version);
        _self.extend_u32(width);
        _self.extend_u32(height);
        _self.extend_u32(color_type as u32);

        if version >= 4 {
            _self.extend_u32(150); // 8-bit gamma integer
        }

        _self
    }

    fn prop_end(&mut self) {
        self.extend_u64(0); // prop : End + size : 0
    }

    pub fn add_properties(&mut self, properties: &Vec<Property>) {
        for property in properties {
            self.extend_u32(property.kind as u32);
            self.extend_u32(property.length as u32); // size
            match &property.payload {
                PropertyPayload::Compression(_value) => {
                    self.data.extend_from_slice(&[_value.to_u8()]);
                },
                PropertyPayload::ResolutionProperty(_value) => {
                    self.extend_u32(_value.xres.to_bits());
                    self.extend_u32(_value.yres.to_bits());
                },
                _ => {}
            }
            self.index += 1;
        }

        self.prop_end();
    }

    pub fn add_layers(&mut self) {
        let mut intermediate_buf = vec!();
        let mut layer_offset_zero_index = 0;

        if self.version >= 11 {
            self.buf_extend_u64( &mut intermediate_buf, &mut layer_offset_zero_index, 0); // layer_offset[n] : 0 = end
            self.buf_extend_u64(&mut intermediate_buf, &mut layer_offset_zero_index, 0); // channel_offset[] = 0 => end
        }
        else {
            self.buf_extend_u32(&mut intermediate_buf, &mut layer_offset_zero_index, 0); // layer_offset[n] : 0 = end
            self.buf_extend_u32(&mut intermediate_buf, &mut layer_offset_zero_index, 0); // channel_offset[] = 0 => end
        }
        self.index += layer_offset_zero_index;

        let mut layer_offset_one_buf = vec!();
        let mut layer_offset_one_index = 0;
        if self.version >= 11 {
            self.buf_extend_u64( &mut layer_offset_one_buf, &mut layer_offset_one_index, self.index as u64 + 8); // layer_offset[0] = le pointer du calque
        }
        else {
            self.buf_extend_u32( &mut layer_offset_one_buf, &mut layer_offset_one_index, self.index + 4); // layer_offset[0] = le pointer du calque
        }
        layer_offset_one_buf.extend_from_slice(&intermediate_buf);
        self.data.extend_from_slice(&layer_offset_one_buf);
        self.index += layer_offset_one_index;
    
        self.extend_u32(1); // layer[0] : width=1
        self.extend_u32(1); // layer[0] : height=1
        self.extend_u32(0); // layer[0] : type=RGB
    
        // layer name :
        self.gimp_string(b"Background");
    
        self.extend_u32(PropertyIdentifier::PropActiveLayer as u32); // prop = 2 : active layer
        self.extend_u32(0);
    
        self.extend_u32(PropertyIdentifier::PropOpacity as u32); // prop : opacity
        self.extend_u32(4); // prop opacity size
        self.extend_u32(RgbaPixel::new(0, 0, 0, 255).to_u32()); // color of opacity = black
    
        self.extend_u32(PropertyIdentifier::PropMode as u32); // prop : Mode
        self.extend_u32(4); // prop mode size
        self.extend_u32(0); // prop mode=normal
    
        // TODO : à améliorer, ça doit être une valeur en float
        self.extend_u32(PropertyIdentifier::PropFloatOpacity as u32); // prop : float opacity
        self.extend_u32(4); // prop float opacity size
        let float_slice = [63, 128, 0, 0];
        self.data.extend_from_slice(&float_slice); // prop float opacity value
        self.index += 4;
    
        self.extend_u32(PropertyIdentifier::PropVisible as u32); // prop : visible
        self.extend_u32(4); // prop visible size
        let float_slice = [0, 0, 0, 1];
        self.data.extend_from_slice(&float_slice); // prop visible value
        self.index += 4;
    
        self.extend_u32(PropertyIdentifier::PropLinked as u32); // prop : linked
        self.extend_u32(4); // prop linked size
        self.extend_u32(0); // prop linked value
    
        self.prop_end();


        // hierarchy offset
        if self.version >= 11 {
            self.extend_u64(self.index as u64 + 16);
            self.extend_u64(0); // mask offset
        } else {
            self.extend_u32(self.index + 8);
            self.extend_u32(0); // mask offset
        }

        // https://testing.developer.gimp.org/core/standards/xcf/#the-hierarchy-structure
        self.extend_u32(1); // width=1
        self.extend_u32(1); // height=1
        self.extend_u32(3); // bpp=3 : RGB color without alpha in 8-bit precision
        if self.version >= 11 {
            self.extend_u64(self.index as u64 + 16); // offset[0]
            self.extend_u64(0);

            self.extend_u32(1); // level[0] width =1
            self.extend_u32(1); // level[0] height =1

            self.extend_u32(0); // ptr : Pointer to tile data

            /*
            self.extend_u32(self.index + 8); // offset
            self.extend_u32(0);

            self.extend_u32(0); // data_offset[0] = 0 => end
            */

            //let slice = [00, 158, 00, 36, 00, 222]; // violet r: 158, g: 23, b: 222  with RLE compression
            let slice = [0, 0, 02, 164, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36]; // violet r: 158, g: 36, b: 222  without compression
            //let slice = [255, 0, 0, 0, 0, 0, 0, 0, 0, 0];
            // \0\0\2\xa4\0\0\0\0\0\0\0\0\0\x9e
            self.data.extend_from_slice(&slice);
        } else {
            self.extend_u32(self.index + 8); // offset[0]

            self.extend_u32(0); // offset[n] = 0 => end

            self.extend_u32(1); // level[0] width =1
            self.extend_u32(1); // level[0] height =1
            self.extend_u32(self.index + 8); // offset= le pointer du contenu
    
            self.extend_u32(0); // data_offset[0] = 0 => end

            //let slice = [00, 158, 00, 36, 00, 222]; // violet r: 158, g: 23, b: 222  with RLE compression
            let slice = [158, 36, 222]; // violet r: 158, g: 36, b: 222  without compression

            // \0\0\2\xa4\0\0\0\0\0\0\0\0\0\x9e
            self.data.extend_from_slice(&slice);
        }
    }
}
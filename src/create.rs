use byteorder::{BigEndian, ByteOrder};

use crate::data::color::ColorType;
use crate::data::layer::Layer;
use crate::data::property::Property;
use crate::data::property::PropertyPayload;
use crate::data::xcf::XcfCompression;
use crate::PropertyIdentifier;
use crate::RgbaPixel;

pub struct XcfCreator {
    pub version: u16,
    pub data: Vec<u8>,
    pub index: u64,
    pub compression: XcfCompression,
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
        data.extend_from_slice(&mut width_buf);
        *index += size as u32;
    }

    fn create_signature(&mut self, gimp_version: u16) {
        let mut signature = format!("gimp xcf v{:03}\0", gimp_version);
        if gimp_version == 1 {
            signature = "gimp xcf file\0".to_string();
        }
        self.data.extend_from_slice(signature.as_bytes());
        self.index += 14;
    }

    fn v10_gimp_string(&mut self, data: &mut Vec<u8>, index: &mut u32, str: &[u8]) {
        let str_count = str.len() as u32;
        self.buf_extend_u32(data, index, str_count + 4);
        data.extend_from_slice(str);
        *index += str_count;
        self.buf_extend_u32(data, index, 0);
    }

    fn gimp_string(&mut self, data: &mut Vec<u8>, index: &mut u32, str: &[u8]) {
        let str_count = str.len() as u32 + 1;
        self.buf_extend_u32(data, index, str_count);
        data.extend_from_slice(str);
        data.extend_from_slice(&[0]);
        *index += str_count;
    }

    pub fn new(version: u16, width: u32, height: u32, color_type: ColorType) -> Self {
        let data = vec![];
        let index = 0;

        let mut _self = XcfCreator {
            version,
            data,
            index,
            compression: XcfCompression::None,
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

    fn prop_end(&mut self, data: &mut Vec<u8>, index: &mut u32) {
        self.buf_extend_u32(data, index, 0);
        self.buf_extend_u32(data, index, 0);
        //self.extend_u64(0); // prop : End + size : 0
    }

    pub fn add_properties(&mut self, properties: &Vec<Property>) {
        let mut _has_compression = false;
        for property in properties {
            self.extend_u32(property.kind as u32);
            match &property.payload {
                PropertyPayload::Compression(_value) => {
                    self.extend_u32(property.length as u32); // size

                    self.data.extend_from_slice(&[_value.to_u8()]);
                    self.index += 1;
                    self.compression = _value.clone();
                    _has_compression = true;
                }
                PropertyPayload::ResolutionProperty(_value) => {
                    self.extend_u32(property.length as u32); // size

                    self.extend_u32(_value.xres.to_bits()); // X resolution in DPI
                    self.extend_u32(_value.yres.to_bits()); // Y resolution in DPI
                }
                PropertyPayload::Tatoo(_value) | PropertyPayload::Unit(_value) => {
                    self.extend_u32(property.length as u32); // size

                    self.extend_u32(*_value);
                }
                PropertyPayload::Parasites(_parasites) => {
                    let mut size: u32 = 0;
                    for parasite in _parasites {
                        size += parasite.name.as_bytes().to_vec().len() as u32;
                        size += parasite.data.as_bytes().to_vec().len() as u32;
                        size += 14;
                    }
                    self.extend_u32(size); // size

                    for parasite in _parasites {
                        self.data.extend_from_slice(&[
                            0,
                            0,
                            0,
                            parasite.name.as_bytes().to_vec().len() as u8 + 1,
                        ]);
                        self.data
                            .extend_from_slice(parasite.name.as_bytes());
                        self.data.extend_from_slice(&[0]);
                        self.index += parasite.name.as_bytes().to_vec().len() as u64 + 5;
                        self.extend_u32(parasite.flags);

                        self.data.extend_from_slice(&[
                            0,
                            0,
                            0,
                            parasite.data.as_bytes().to_vec().len() as u8 + 1,
                        ]);
                        self.data
                            .extend_from_slice(parasite.data.as_bytes());
                        self.data.extend_from_slice(&[0]);
                        self.index += parasite.data.as_bytes().to_vec().len() as u64 + 5;
                    }
                }
                _ => {
                    self.extend_u32(property.length as u32); // size
                }
            }
        }
        if self.version > 10 && !_has_compression {
            self.extend_u32(PropertyIdentifier::PropCompression as u32);
            self.extend_u32(1); // size
            self.data.extend_from_slice(&[XcfCompression::Rle as u8]);
            self.index += 1;
            self.compression = XcfCompression::Rle;

            // resolution
            self.extend_u32(PropertyIdentifier::PropResolution as u32);
            self.extend_u32(8); // size
            let value: f32 = 300.0;
            self.extend_u32(value.to_bits()); // X resolution in DPI
            self.extend_u32(value.to_bits()); // Y resolution in DPI

            // tatoo
            self.extend_u32(PropertyIdentifier::PropTattoo as u32);
            self.extend_u32(4); // size
            self.extend_u32(2);

            // unit
            self.extend_u32(PropertyIdentifier::PropUnit as u32);
            self.extend_u32(4); // size
            self.extend_u32(1);

            // parasites
            self.extend_u32(PropertyIdentifier::PropParasites as u32);
            self.extend_u32(238); // size

            self.data.extend_from_slice(&[0, 0, 0, 13]);
            self.data.extend_from_slice(b"gimp-comment");
            self.data.extend_from_slice(&[0]);
            self.index += 17;
            self.extend_u32(1);

            self.data.extend_from_slice(&[0, 0, 0, 13]);
            self.data.extend_from_slice(b"Test Comment");
            self.data.extend_from_slice(&[0]);
            self.index += 17;

            self.data.extend_from_slice(&[0, 0, 0, 16]);
            self.data.extend_from_slice(b"gimp-image-grid");
            self.data.extend_from_slice(&[0]);
            self.index += 20;
            self.extend_u32(1);

            self.data.extend_from_slice(&[0, 0, 0, 172]);
            self.data.extend_from_slice(b"(style solid)\n(fgcolor (color-rgba 0 0 0 1))\n(bgcolor (color-rgba 1 1 1 1))\n(xspacing 10)\n(yspacing 10)\n(spacing-unit inches)\n(xoffset 0)\n(yoffset 0)\n(offset-unit inches)\n");
            self.data.extend_from_slice(&[0]);
            self.index += 176;
        }

        // TODO : replace by : self.propend()
        self.extend_u32(0);
        self.extend_u32(0);
    }

    fn _add_layers_properties(&mut self, data: &mut Vec<u8>, index: &mut u32, layers_properties: &Vec<Property>) {
        for layer_property in layers_properties {
            self.buf_extend_u32(data, index, layer_property.kind as u32);
            self.buf_extend_u32(data, index, layer_property.length as u32); // size
            match &layer_property.payload {
                PropertyPayload::Compression(_value) => {
                    data.extend_from_slice(&[_value.to_u8()]);
                    *index += 1;
                }
                PropertyPayload::OpacityLayer(_value) => {
                    data
                        .extend_from_slice(&[_value.r(), _value.g(), _value.b(), _value.a()]);
                    *index += 4;
                }
                PropertyPayload::FloatOpacityLayer() => {
                    // TODO : à améliorer, ça doit être une valeur en float
                    let float_slice = [63, 128, 0, 0];
                    data.extend_from_slice(&float_slice); // prop float opacity value
                    *index += 4;
                }
                PropertyPayload::VisibleLayer() => {
                    let float_slice = [0, 0, 0, 1];
                    data.extend_from_slice(&float_slice); // prop visible value
                    *index += 4;
                }
                PropertyPayload::OffsetsLayer(_offset_x, _offset_y) => {
                    self.buf_extend_u32(data, index, *_offset_x);
                    self.buf_extend_u32(data, index, *_offset_y);
                }
                PropertyPayload::LinkedLayer(_value)
                | PropertyPayload::ColorTagLayer(_value)
                | PropertyPayload::LockContentLayer(_value)
                | PropertyPayload::LockAlphaLayer(_value)
                | PropertyPayload::LockPositionLayer(_value)
                | PropertyPayload::ApplyMaskLayer(_value)
                | PropertyPayload::EditMaskLayer(_value)
                | PropertyPayload::ShowMaskLayer(_value)
                | PropertyPayload::ModeLayer(_value)
                | PropertyPayload::BlendSpaceLayer(_value)
                | PropertyPayload::CompositeSpaceLayer(_value)
                | PropertyPayload::CompositeModeLayer(_value)
                | PropertyPayload::Tatoo(_value) => {
                    self.buf_extend_u32(data, index, *_value);
                }
                _ => {}
            }
        }
        if self.version > 10 && layers_properties.iter().len() == 0 {
            // active
            self.buf_extend_u32(data, index, PropertyIdentifier::PropActiveLayer as u32);
            self.buf_extend_u32(data, index, 0);
            // opacity
            self.buf_extend_u32(data, index, PropertyIdentifier::PropOpacity as u32);
            self.buf_extend_u32(data, index, 4);
            data.extend_from_slice(&[0, 0, 0, 255]);
            *index += 4;
            // float opacity
            self.buf_extend_u32(data, index, PropertyIdentifier::PropFloatOpacity as u32);
            self.buf_extend_u32(data, index, 4);
            let float_slice = [63, 128, 0, 0];
            data.extend_from_slice(&float_slice); // prop float opacity value
            *index += 4;
            // visible
            self.buf_extend_u32(data, index, PropertyIdentifier::PropVisible as u32);
            self.buf_extend_u32(data, index, 4);
            let float_slice = [0, 0, 0, 1];
            data.extend_from_slice(&float_slice); // prop visible value
            *index += 4;

            // linked
            self.buf_extend_u32(data, index, PropertyIdentifier::PropLinked as u32);
            self.buf_extend_u32(data, index, 4);
            self.buf_extend_u32(data, index, 0);

            // color tag
            self.buf_extend_u32(data, index, PropertyIdentifier::PropColorTag as u32);
            self.buf_extend_u32(data, index, 4);
            self.buf_extend_u32(data, index, 0);

            // lock content
            self.buf_extend_u32(data, index, PropertyIdentifier::PropLockContent as u32);
            self.buf_extend_u32(data, index, 4);
            self.buf_extend_u32(data, index, 0);

            // lock alpha
            self.buf_extend_u32(data, index, PropertyIdentifier::PropLockAlpha as u32);
            self.buf_extend_u32(data, index, 4);
            self.buf_extend_u32(data, index, 0);

            // lock position
            self.buf_extend_u32(data, index, PropertyIdentifier::PropLockPosition as u32);
            self.buf_extend_u32(data, index, 4);
            self.buf_extend_u32(data, index, 0);

            // apply mask
            self.buf_extend_u32(data, index, PropertyIdentifier::PropApplyMask as u32);
            self.buf_extend_u32(data, index, 4);
            self.buf_extend_u32(data, index, 0);

            // edit mask
            self.buf_extend_u32(data, index, PropertyIdentifier::PropEditMask as u32);
            self.buf_extend_u32(data, index, 4);
            self.buf_extend_u32(data, index, 0);

            // show mask
            self.buf_extend_u32(data, index, PropertyIdentifier::PropShowMask as u32);
            self.buf_extend_u32(data, index, 4);
            self.buf_extend_u32(data, index, 0);

            // offsets
            self.buf_extend_u32(data, index, PropertyIdentifier::PropOffsets as u32);
            self.buf_extend_u32(data, index, 8);
            self.buf_extend_u32(data, index, 0);
            self.buf_extend_u32(data, index, 0);

            // if version >= 11, than the layer mode must be the new normal mode (not legacy)
            self.buf_extend_u32(data, index, PropertyIdentifier::PropMode as u32);
            self.buf_extend_u32(data, index, 4); // size
            self.buf_extend_u32(data, index, 28); // mode normal after version 10

            // blend space
            self.buf_extend_u32(data, index, PropertyIdentifier::PropBlendSpace as u32);
            self.buf_extend_u32(data, index, 4);
            self.buf_extend_u32(data, index, 0);

            // composite space
            self.buf_extend_u32(data, index, PropertyIdentifier::PropCompositeSpace as u32);
            self.buf_extend_u32(data, index, 4);
            self.buf_extend_u32(data, index, u32::MAX);

            // composite mode
            self.buf_extend_u32(data, index, PropertyIdentifier::PropCompositeMode as u32);
            self.buf_extend_u32(data, index, 4);
            self.buf_extend_u32(data, index, u32::MAX);

            // tatoo
            self.buf_extend_u32(data, index, PropertyIdentifier::PropTattoo as u32);
            self.buf_extend_u32(data, index, 4);
            self.buf_extend_u32(data, index, 2);
        }
        self.prop_end(data, index);
    }

    fn _add_layers_v10(&mut self, _layers: &[Layer]) {
        let mut intermediate_buf = vec![];
        let mut layer_offset_zero_index = 0;

        self.buf_extend_u32(&mut intermediate_buf, &mut layer_offset_zero_index, 0); // layer_offset[n] : 0 = end
        self.buf_extend_u32(&mut intermediate_buf, &mut layer_offset_zero_index, 0); // channel_offset[] = 0 => end

        self.index += layer_offset_zero_index as u64;

        let mut layer_offset_one_buf = vec![];
        let mut layer_offset_one_index = 0;

        self.buf_extend_u32(
            &mut layer_offset_one_buf,
            &mut layer_offset_one_index,
            self.index as u32 + 4,
        ); // layer_offset[0] = le pointer du calque

        layer_offset_one_buf.extend_from_slice(&intermediate_buf);
        self.data.extend_from_slice(&layer_offset_one_buf);
        self.index += layer_offset_one_index as u64;

        self.extend_u32(1); // layer[0] : width=1
        self.extend_u32(1); // layer[0] : height=1
        self.extend_u32(0); // layer[0] : type=RGB

        // layer name :
        let mut layer_name_data = vec!();
        let mut layer_name_len = 0;
        self.v10_gimp_string(&mut layer_name_data, &mut layer_name_len, b"Background");
        self.data.extend_from_slice(&layer_name_data);
        self.index += layer_name_len as u64;

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


        // TODO : replace by : self.propend()
        self.extend_u32(0);
        self.extend_u32(0);

        // hierarchy offset
        self.extend_u32(self.index as u32 + 8);
        self.extend_u32(0); // mask offset

        // https://testing.developer.gimp.org/core/standards/xcf/#the-hierarchy-structure
        self.extend_u32(1); // width=1
        self.extend_u32(1); // height=1
        self.extend_u32(3); // bpp=3 : RGB color without alpha in 8-bit precision

        self.extend_u32(self.index as u32 + 8); // offset[0]

        self.extend_u32(0); // offset[n] = 0 => end

        self.extend_u32(1); // level[0] width =1
        self.extend_u32(1); // level[0] height =1
        self.extend_u32(self.index as u32 + 8); // offset= le pointer du contenu

        self.extend_u32(0); // data_offset[0] = 0 => end

        //let slice = [00, 158, 00, 36, 00, 222]; // violet r: 158, g: 23, b: 222  with RLE compression
        let slice = [158, 36, 222]; // violet r: 158, g: 36, b: 222  without compression

        // \0\0\2\xa4\0\0\0\0\0\0\0\0\0\x9e
        self.data.extend_from_slice(&slice);
    }

    pub fn add_layers(&mut self, layers: &Vec<Layer>) {
        if self.version < 11 {
            self._add_layers_v10(layers);
            return;
        }
        let nb_layers = layers.iter().len();

        let mut layer_data = vec!();
        let mut layer_len = 0;
        let mut index = 0;
        for layer in layers {
            index += 1;

            // Each layers is 8 bits + 8 bits for close layers + 8 bits for close channels
            self.extend_u64(self.index + (nb_layers as u64 - index + 1) * 8 + 8 + layer_len as u64 + 8); // layer_offset[index -1]

            self.buf_extend_u32(&mut layer_data, &mut layer_len, layer.width);
            self.buf_extend_u32(&mut layer_data, &mut layer_len, layer.height);
            self.buf_extend_u32(&mut layer_data, &mut layer_len, layer.kind.kind.clone() as u32);

            // layer name
            self.gimp_string(&mut layer_data, &mut layer_len, layer.name.as_bytes());
            // layer properties
            self._add_layers_properties(&mut layer_data, &mut layer_len, &layer.properties);

            let mut hierarchy_data = vec![];
            let mut hierarchy_len = 0;
            // https://testing.developer.gimp.org/core/standards/xcf/#the-hierarchy-structure
            self.buf_extend_u32(&mut hierarchy_data, &mut hierarchy_len,layer.pixels.width); // width=1
            self.buf_extend_u32(&mut hierarchy_data, &mut hierarchy_len,layer.pixels.height); // height=1
            self.buf_extend_u32(&mut hierarchy_data, &mut hierarchy_len,3); // bpp=3 : RGB color without alpha in 8-bit precision

            let mut offset_data = vec![];
            let mut offset_len = 0;
            self.buf_extend_u32(&mut offset_data, &mut offset_len,layer.pixels.width); // level[0] width
            self.buf_extend_u32(&mut offset_data, &mut offset_len,layer.pixels.height); // level[0] height
            self.buf_extend_u32(&mut offset_data, &mut offset_len,0); // ptr : Pointer to tile data

            let mut offset_index = self.index + (nb_layers as u64 - index + 1) * 8 + 8 + layer_len as u64 + hierarchy_len as u64 + 32;
            if index != 1 {
                offset_index = 963;
            }
            println!("offset[0] : {} => index {}, nb_layer {}, layer_len {}, hierarchy_len {}", offset_index, self.index, nb_layers, layer_len, hierarchy_len);
            self.buf_extend_u64(&mut hierarchy_data, &mut hierarchy_len, offset_index); // offset[0]
            self.buf_extend_u64(&mut hierarchy_data, &mut hierarchy_len,0); // offset[1]
            hierarchy_data.extend_from_slice(&offset_data);
            hierarchy_len += offset_len;

            // hierarchy offset
            let hierarchy_index = self.index + layer_len as u64 + 32;
            self.buf_extend_u64(&mut layer_data, &mut layer_len, hierarchy_index); // hierarchy_ofs=
            self.buf_extend_u64(&mut layer_data, &mut layer_len,0); // layer mask offset
            layer_data.extend_from_slice(&hierarchy_data);
            layer_len += hierarchy_len;

            if self.compression == XcfCompression::Rle {
                let pixels_index = self.index + (nb_layers as u64 - index + 1) * 8 + 8 + layer_len as u64 + 12;
                println!(">>>>> pixels_index {} {} {}", self.index + (nb_layers as u64 - index + 1) * 8 + 8 , layer_len, pixels_index);
                self.buf_extend_u32(&mut layer_data, &mut layer_len, pixels_index as u32);
                layer_data.extend_from_slice(&[0, 0, 0, 0, 0, 0, 0, 0]);
                let nb_of_pixels = layer.pixels.pixels.iter().len() as u32;
                let nb_pixels_of_layers = layer.pixels.width * layer.pixels.height;
                if nb_pixels_of_layers != nb_of_pixels {
                    panic!(
                        "Number of pixels on the layers {} and pixels {} aren't equals",
                        nb_pixels_of_layers, nb_of_pixels
                    );
                }
                let mut buffer_r = vec![];
                let mut buffer_g = vec![];
                let mut buffer_b = vec![];
                for pixel in &layer.pixels.pixels {
                    buffer_r.push(pixel.r());
                    buffer_g.push(pixel.g());
                    buffer_b.push(pixel.b());
                }
                let mut buffer = vec![];

                println!("buffer_r {:?}", &buffer_r);
                println!("rle_r {:?}", rle_compress(&buffer_r));
                println!("buffer_g {:?}", &buffer_g);
                println!("rle_g {:?}", rle_compress(&buffer_g));
                println!("buffer_b {:?}", &buffer_b);
                println!("rle_b {:?}", rle_compress(&buffer_b));
            
                buffer.extend(rle_compress(&buffer_r));
                buffer.extend(rle_compress(&buffer_g));
                buffer.extend(rle_compress(&buffer_b));
                layer_data.extend_from_slice(&buffer);
                //self.data.extend_from_slice(&layer_data);
                //self.index += layer_len as u64;
            } else {
                panic!("not implemented");
            }
        }
        self.extend_u64(0); // layer_offset[1] = 0
        self.extend_u64(0); // channel_offset[0] = 0
        //return;
        self.data.extend_from_slice(&layer_data);
        self.index += layer_len as u64;
    }
}

// https://testing.developer.gimp.org/core/standards/xcf/#rle-compressed-tile-data
pub fn rle_compress(data: &Vec<u8>) -> Vec<u8> {
    let mut compress_data = vec![];
    let mut last_byte = None;
    let mut short_identical_len = 0;
    let mut verbatim = vec![];
    let mut count = 0;
    for byte in data {
        count += 1;
        //let verbatim_len = verbatim.iter().len();
        if let Some(val) = last_byte {
            if *byte == val {
                /*
                if short_identical_len > 1 && verbatim_len > 0 {
                    compress_data.push((256 - verbatim_len) as u8);
                    compress_data.extend_from_slice(&verbatim);
                    verbatim = vec![];
                }
                */
                verbatim.push(val);
                short_identical_len += 1;
                continue;
            }
        }
        if count == 3 && short_identical_len + 2 == count {
            compress_data.push(short_identical_len as u8);
            compress_data.push(last_byte.unwrap());
            verbatim = vec![];
        }
        /*
        if short_identical_len > 0 && (verbatim_len - short_identical_len) > 2 {
            println!("pop {:?} <=> {:?}", verbatim, compress_data);
            //compress_data.pop();
            compress_data.push(short_identical_len as u8);
            compress_data.push(last_byte.unwrap());
            verbatim = vec![];
        }
        */
        short_identical_len = 0;
        verbatim.push(*byte);
        last_byte = Some(*byte);
    }

    if short_identical_len == 0 {
        if verbatim.iter().len() == 1 {
            compress_data.push(0);
        } else {
            compress_data.push((256 - verbatim.iter().len()) as u8);
        }
        compress_data.extend_from_slice(&verbatim);
    }
    compress_data
}

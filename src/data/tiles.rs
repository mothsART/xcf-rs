use crate::Layer;

#[derive(Debug, PartialEq)]
pub struct Tiles {
    pub nb_width: u32,
    pub nb_height: u32,
    pub nb: u32,
}

impl Tiles {
    pub fn new(layer: &Layer) -> Self {
        let nb_width = (layer.width as f32 / 64.0).ceil() as u32;
        let nb_height = (layer.height as f32 / 64.0).ceil() as u32;
        Tiles {
            nb_width,
            nb_height,
            nb: nb_width * nb_height
        }
    }
}
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RgbaPixel(pub [u8; 4]);

impl RgbaPixel {
    pub fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        RgbaPixel([red, green, blue, alpha])
    }

    pub fn r(&self) -> u8 {
        self.0[0]
    }

    pub fn g(&self) -> u8 {
        self.0[1]
    }

    pub fn b(&self) -> u8 {
        self.0[2]
    }

    pub fn a(&self) -> u8 {
        self.0[3]
    }

    pub fn to_u32(&self) -> u32 {
        self.r() as u32 + self.g() as u32 + self.b() as u32 + self.a() as u32
    }
}

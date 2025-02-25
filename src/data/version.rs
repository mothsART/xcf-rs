#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Version(pub u16);

impl Version {
    pub fn num(self) -> u16 {
        self.0
    }

    pub fn bytes_per_offset(self) -> usize {
        if self.0 >= 11 {
            8
        } else {
            4
        }
    }
}

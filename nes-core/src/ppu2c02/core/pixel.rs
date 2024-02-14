#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct Pixel {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl Pixel {
    pub fn set_grayscale(&mut self, v: u8) {
        self.red = v;
        self.green = v;
        self.blue = v;
    }
}

impl Default for Pixel {
    fn default() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
            alpha: 255,
        }
    }
}

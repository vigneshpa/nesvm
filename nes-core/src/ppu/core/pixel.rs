static PALETTE: &'static [u8] = include_bytes!("nespalette.pal");

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct Pixel {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl Pixel {
    pub fn grayscale(&mut self, v: u8) {
        self.red = v;
        self.green = v;
        self.blue = v;
    }

    /// Applies the color (0 to 63) to from the system pallete
    pub fn color(&mut self, color: u8) {
        let color = (color & 0x3F) as usize;
        self.red = PALETTE[color * 3 + 0];
        self.green = PALETTE[color * 3 + 1];
        self.blue = PALETTE[color * 3 + 2];
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

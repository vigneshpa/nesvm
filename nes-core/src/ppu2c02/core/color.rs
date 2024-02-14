use super::pixel::Pixel;

static PALETTE: &'static [u8] = include_bytes!("nespalette.pal");

pub fn apply(pixel: &mut Pixel, color: u8) {
    let color = (color & 0x3F) as usize;
    pixel.red = PALETTE[color * 3 + 0];
    pixel.green = PALETTE[color * 3 + 1];
    pixel.blue = PALETTE[color * 3 + 2];
}
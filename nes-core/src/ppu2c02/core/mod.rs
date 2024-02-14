use crate::{Bus, Tick};
pub mod pixel;
mod color;
use pixel::Pixel;

const SCREEN_W: usize = 256;
const SCREEN_H: usize = 240;

const SCREEN_SZ: usize = SCREEN_W * SCREEN_H;

pub trait VideoBackend {
    fn render(&mut self, fb: &[Pixel]) -> ();
}

pub struct Core<B: Bus> {
    bus: B,
    oam: Box<[u8; 512]>,
    fb: [Pixel; SCREEN_SZ],
    video_backend: Box<dyn VideoBackend>,
}

impl<B: Bus> Core<B> {
    pub fn new(bus: B, video_backend: impl VideoBackend + 'static) -> Self {
        Self {
            bus,
            oam:Box::new([0u8; 512]),
            video_backend: Box::new(video_backend),
            fb: [Pixel::default(); SCREEN_SZ],
        }
    }

    pub fn render(&mut self) {
        self.video_backend.render(&self.fb);
    }

    pub fn draw_sprite(&mut self, target_x: usize, target_y: usize , tile_number: u8, is_second_half: bool) {
        let mut sprite_data = [0u8; 16];
        let base = match is_second_half {
            false => 0x0000,
            true => 0x1000,
        };
        self.bus.read_to_slice(base + (tile_number as u16 * 16), &mut sprite_data);
        for i in 0..8 {
            let rowa = sprite_data[i];
            let rowb = sprite_data[8 + i];
            for j in 0..8 {
                let n = bitat(rowa, 7 - j) + bitat(rowb, 7 - j);
                self.color_pixel(j + target_x, i + target_y, n, 0);
            }
        }
    }

    pub fn color_pixel(&mut self, x: usize, y:usize, n: u8, p:u8) {
        let pixel = &mut self.fb[y * SCREEN_W + x];
        pixel.set_grayscale(n * 85);
        // let mut color = 0x3F00u16;
        // if n != 0 {
        //     color += (n as u16) + (p as u16) * 4;
        // }
        // let color = self.bus.read(color);
        // color::apply(pixel, color);
    }

    #[allow(dead_code)]
    pub fn draw_debug(&mut self) {
        for i in 0..16u8 {
            for j in 0..16u8 {
                self.draw_sprite((j * 8) as usize, (i * 8) as usize, i * 16 + j, false);
            }
        }
        for i in 0..16u8 {
            for j in 0..16u8 {
                self.draw_sprite(((j + 16) * 8) as usize, (i * 8) as usize, i * 16 + j, true);
            }
        }
    }
}

fn bitat(data:u8, i:usize) -> u8 {
    ((0b1 << i) & data) >> i
}

impl<B: Bus> Tick for Core<B> {
    fn tick(&mut self) -> u8 {
        self.draw_debug();
        self.render();
        // todo!()
        0
    }
}

impl<B: Bus> Bus for Core<B> {
    fn read(&self, _address: u16) -> u8 {
        todo!()
    }

    fn write(&mut self, _address: u16, _data: u8) {
        todo!()
    }
}
use crate::{Bus, Tick};
pub mod pixel;
use pixel::Pixel;

const SCREEN_W: usize = 256;
const SCREEN_H: usize = 240;

const SCREEN_SZ: usize = SCREEN_W * SCREEN_H;

pub trait VideoBackend {
    fn render(&mut self, fb: &[Pixel]) -> ();
}

pub struct Core<B: Bus> {
    bus: B,
    // oam: Box<[u8; 512]>,
    fb: [Pixel; SCREEN_SZ],
    video_backend: Box<dyn VideoBackend>,
}

impl<B: Bus> Core<B> {
    pub fn new(bus: B, video_backend: impl VideoBackend + 'static) -> Self {
        Self {
            bus,
            // oam:Box::new([0u8; 512]),
            video_backend: Box::new(video_backend),
            fb: [Pixel::default(); SCREEN_SZ],
        }
    }

    pub fn render(&mut self) {
        self.draw_background();
        self.draw_debug();
        self.video_backend.render(&self.fb);
    }

    fn draw_background(&mut self) {
        let name_table = 0x2000;
        for i in 0..30 {
            for j in 0..32 {
                let pattern_idx = self.bus.read(name_table + i*32 + j);
                self.draw_sprite(j * 8, i * 8, pattern_idx, false);
            }
        }
    }

    fn draw_sprite(&mut self, target_x: u16, target_y: u16 , tile_number: u8, is_second_half: bool) {
        let mut sprite_data = [0u8; 16];
        let base = match is_second_half {
            false => 0x0000,
            true => 0x1000,
        };
        self.bus.read_to_slice(base + (tile_number as u16 * 16), &mut sprite_data);
        for i in 0..8 {
            let rowa = sprite_data[i as usize];
            let rowb = sprite_data[8 + i as usize];
            for j in 0..8 {
                let n = (bitat(rowa, 7 - j) << 1) + bitat(rowb, 7 - j);
                self.color_pixel(j + target_x, i + target_y, n, 0);
            }
        }
    }

    fn color_pixel(&mut self, x: u16, y:u16, n: u8, p:u8) {
        let pixel_idx = y as usize * SCREEN_W + x as usize;
        let pixel = &mut self.fb[pixel_idx];
        let mut color = 0x3F00u16;
        if n != 0 {
            color += (n as u16) + (p as u16) * 4;
        }
        let color = self.bus.read(color);
        pixel.color(color);
        pixel.grayscale(n * 85);
    }

    #[allow(dead_code)]
    fn draw_debug(&mut self) {
        for i in 0..16u8 {
            for j in 0..16u8 {
                self.draw_sprite((j * 8) as u16, (i * 8) as u16, i * 16 + j, false);
            }
        }
        for i in 0..16u8 {
            for j in 0..16u8 {
                self.draw_sprite(((j + 16) * 8) as u16, (i * 8) as u16, i * 16 + j, true);
            }
        }
    }
}

fn bitat(data:u8, i:u16) -> u8 {
    ((0b1 << i) & data) >> i
}

impl<B: Bus> Tick for Core<B> {
    fn tick(&mut self) -> u8 {
        self.render();
        0
    }
}

impl<B: Bus> Bus for Core<B> {
    fn read(&self, _address: u16) -> u8 {
        0
    }

    fn write(&mut self, _address: u16, _data: u8) {
    }
}
use crate::{Bus, Tick};

pub trait VideoBackend {
    fn render(&mut self, fb: &[u8]) -> ();
}

pub struct Core<B: Bus> {
    bus: B,
    video_backend: Box<dyn VideoBackend>
}

impl<B: Bus> Core<B> {
    pub fn new(bus: B, video_backend: impl VideoBackend + 'static) -> Self {
        Self {
            bus,
            video_backend: Box::new(video_backend)
        }
    }
}

impl<B: Bus> Tick for Core<B> {
    fn tick(&mut self) -> u8 {
        let mut buf = vec![0u8; 256 * 240 * 4];
        for i in 0..buf.len() {
            if i % 4 == 3 {
                buf[i] = 255;
            }
        }
        println!("Rendering Black Frame");
        self.video_backend.render(&buf);
        1
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
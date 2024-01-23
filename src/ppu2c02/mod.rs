use crate::{Bus, Tick};

pub struct PPU {}

impl PPU {
    pub fn new() -> Self {
        Self {}
    }
}

impl Tick for PPU {
    fn tick(&mut self) {
    }
}

// PPU registers
impl Bus for PPU {
    fn read(&self, _address: u16) -> u8 {
        todo!()
    }

    fn write(&mut self, _address: u16, _data: u8) -> () {
        todo!()
    }
}

impl Clone for PPU {
    fn clone(&self) -> Self {
        Self {  }
    }
}
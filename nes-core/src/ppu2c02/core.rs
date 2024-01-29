use crate::{Bus, Tick};

pub struct Core<B: Bus> {
    bus: B
}

impl<B: Bus> Core<B> {
    pub fn new(bus: B) -> Self {
        Self {
            bus
        }
    }
}

impl<B: Bus> Tick for Core<B> {
    fn tick(&mut self) -> u8 {
        todo!()
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
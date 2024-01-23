#![allow(dead_code)]

pub mod cpu6502;
pub mod ppu2c02;
pub mod motherboard;
pub mod gamepack;
pub mod utils;

pub trait Tick {
    fn tick(&mut self);
}

pub trait Bus {
    fn read(&self, address: u16) -> u8;
    fn write(&mut self, address: u16, data: u8) -> ();
}

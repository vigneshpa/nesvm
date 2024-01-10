#![allow(dead_code)]

pub mod cpu6502;
pub mod motherboard;
pub mod utils;

pub trait Tick {
    fn tick(&mut self);
}

pub trait Bus {
    fn get(&self, address: u16) -> u8;
    fn set(&mut self, address: u16, data: u8) -> ();
}

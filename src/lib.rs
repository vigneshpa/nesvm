#![allow(dead_code)]

pub mod cpu6502;
pub mod motherboard;
pub mod utils;
pub trait Tick {
    fn tick(&mut self);
}

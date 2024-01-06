#![allow(dead_code)]

pub mod cpu6502;
pub mod motherboard;
pub mod utils;
pub trait Tickable {
    fn tick(&mut self);
}

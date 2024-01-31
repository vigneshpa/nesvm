#![allow(dead_code)]
#![allow(unused)]
use crate::Bus;

pub fn decode_nes_file(_file:&[u8]) -> Box<dyn Mapper> {
    // Box::new(Mapper000 {
    //     pgr_rom: Vec::new(),
    //     chr_rom: Vec::new(),
    // })
    todo!()
}

pub trait Mapper: Bus {
    fn ppu_read(&self, address: u16) -> u8;
    fn ppu_write(&mut self, address: u16, data: u8) -> ();
}

pub struct Mapper000 {
    pgr_rom: Vec<[u8; 0x4000]>,
    chr_rom: Vec<[u8; 0x2000]>,
}

impl Bus for Mapper000 {
    fn read(&self, address: u16) -> u8 {
        todo!()
    }

    fn write(&mut self, address: u16, data: u8) -> () {
        todo!()
    }
}

impl Mapper for Mapper000 {
    fn ppu_read(&self, address: u16) -> u8 {
        todo!()
    }

    fn ppu_write(&mut self, address: u16, data: u8) -> () {
        todo!()
    }
}
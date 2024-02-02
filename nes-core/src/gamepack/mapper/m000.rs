use crate::Bus;
use super::Mapper;

// NROM
pub struct Mapper000 {
    pub pgr_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
}

impl Bus for Mapper000 {
    fn read(&self, mut address: u16) -> u8 {
        if 0x8000 <= address {
            address -= 0x8000;
            address &= 0b0011_1111;
            self.pgr_rom[address as usize]
        } else { 0 }
    }

    fn write(&mut self, mut address: u16, data: u8) -> () {
        if 0x8000 <= address {
            address -= 0x8000;
            address &= 0b0011_1111;
            self.pgr_rom[address as usize] = data
        }
    }
}

impl Mapper for Mapper000 {
    fn ppu_read(&self, address: u16) -> u8 {
        self.chr_rom[address as usize]
    }

    fn ppu_write(&mut self, address: u16, data: u8) -> () {
        self.chr_rom[address as usize] = data
    }
}
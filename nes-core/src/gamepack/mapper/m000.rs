use super::Mapper;
use crate::Bus;

// NROM
pub struct Mapper000 {
    pgr_rom: PGR,
    chr_rom: CHR,
}

impl Mapper000 {
    pub fn new(pgr_rom: Vec<u8>, chrr_rom: Vec<u8>) -> Self {
        Self {
            pgr_rom: PGR(pgr_rom),
            chr_rom: CHR(chrr_rom),
        }
    }
}

struct PGR(Vec<u8>);
impl Bus for PGR {
    fn read(&self, mut address: u16) -> u8 {
        if 0x8000 <= address {
            address -= 0x8000;
            address &= 0b0011_1111;
            self.0[address as usize]
        } else {
            0
        }
    }

    fn write(&mut self, mut address: u16, data: u8) -> () {
        if 0x8000 <= address {
            address -= 0x8000;
            address &= 0b0011_1111;
            self.0[address as usize] = data
        }
    }
}

struct CHR(Vec<u8>);
impl Bus for CHR {
    fn read(&self, address: u16) -> u8 {
        self.0[address as usize]
    }

    fn write(&mut self, address: u16, data: u8) -> () {
        self.0[address as usize] = data
    }
}

impl Mapper for Mapper000 {
    fn cpu_bus(&mut self) -> &mut dyn Bus {
        &mut self.pgr_rom
    }

    fn ppu_bus(&mut self) -> &mut dyn Bus {
        &mut self.chr_rom
    }
}

use crate::{gamepack::GamePack, ppu2c02::PPU, Bus};

use super::{ppubus::PpuBus, ram::RAM};

pub struct CpuBus {
    memory: Box<[u8; 0x0800]>,
    ppu: PPU<PpuBus>,
    apu: RAM,
    gamepack: GamePack,
}

impl CpuBus {
    pub fn new(ppu: PPU<PpuBus>, apu: RAM, gamepack: GamePack) -> Self {
        Self {
            memory: Box::new([0u8; 0x0800]),
            ppu,
            apu,
            gamepack,
        }
    }
}

impl Bus for CpuBus {
    fn read(&self, address: u16) -> u8 {
        if address <= 0x1FFF {
            self.memory[(address & 0x07FF) as usize]
        } else if address <= 0x3FFF {
            self.ppu.read(address & 0x0007)
        } else if address <= 0x4017 {
            self.apu.read(address & 0x0017)
        } else if address <= 0x401F {
            0
        }
        else {
            self.gamepack.read(address)
        }
    }

    fn write(&mut self, address: u16, data: u8) -> () {
        if address <= 0x1FFF {
            self.memory[(address & 0x07FF) as usize] = data;
        } else if address <= 0x3FFF {
            self.ppu.write(address & 0x0007, data);
        } else if address <= 0x4017 {
            self.apu.write(address & 0x0017, data);
        } else if address <= 0x401F {
        }
        else {
            self.gamepack.write(address, data);
        }
    }
}
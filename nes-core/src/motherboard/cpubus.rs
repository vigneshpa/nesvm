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
        match address {
            0x0000..=0x1FFF => self.memory[(address & 0x07FF) as usize],
            0x2000..=0x3FFF => self.ppu.read(address & 0x0007),
            0x4000..=0x4017 => self.apu.read(address & 0x0017),
            0x4018..=0x401F => 0,
            _ => self.gamepack.read(address),
        }
    }

    fn write(&mut self, address: u16, data: u8) -> () {
        match address {
            0x0000..=0x1FFF => self.memory[(address & 0x07FF) as usize] = data,
            0x2000..=0x3FFF => self.ppu.write(address & 0x0007, data),
            0x4000..=0x4017 => self.apu.write(address & 0x0017, data),
            0x4018..=0x401F => {},
            _ => self.gamepack.write(address, data),

        }
    }
}
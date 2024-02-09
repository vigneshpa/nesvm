use cpu6502::CPU;
use gamepack::GamePack;
use motherboard::{cpubus::CpuBus, ppubus::PpuBus, ram::RAM};
use ppu2c02::PPU;

pub mod cpu6502;
pub mod gamepack;
pub mod motherboard;
pub mod ppu2c02;
pub mod utils;
pub trait Tick {
    fn tick(&mut self) -> u8;
}

pub trait Bus {
    fn read(&self, address: u16) -> u8;
    fn write(&mut self, address: u16, data: u8) -> ();
}

pub struct Core {
    cpu: CPU<CpuBus>,
    ppu: PPU<PpuBus>,
}

// TODO: Fix to proper cycles implementation
impl Tick for Core {
    fn tick(&mut self) -> u8 {
        let mut cycles = 0;
        cycles += self.ppu.tick();
        cycles += self.ppu.tick();
        cycles += self.ppu.tick();
        cycles += self.cpu.tick();
        cycles
    }
}

impl Core {
    pub fn new(nes_file: &[u8]) -> Self {

        let gamepack = GamePack::new(nes_file);

        let ppu_bus = PpuBus::new(gamepack.clone());
        let ppu = PPU::new(ppu_bus);

        let apu = RAM::new(0x0018);
        let cpu_bus = CpuBus::new(ppu.clone(), apu, gamepack);

        let cpu = CPU::new(cpu_bus, 0);

        Self { cpu, ppu }
    }
}

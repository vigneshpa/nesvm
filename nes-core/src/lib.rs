use cpu6502::CPU;
use gamepack::GamePack;
use motherboard::{cpubus::CpuBus, ppubus::PpuBus, ram::RAM};
use ppu2c02::{VideoBackend, PPU};

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
    fn read_to_slice(&self, mut address: u16, slice:&mut [u8]) {
        for el in slice {
            *el = self.read(address);
            address += 1;
        }
    }
}

pub struct Emulator {
    cpu: CPU<CpuBus>,
    ppu: PPU<PpuBus>,
}

// TODO: Fix to proper cycles implementation
impl Tick for Emulator {
    fn tick(&mut self) -> u8 {
        let mut cycles = 0;
        cycles += self.ppu.tick();
        cycles += self.ppu.tick();
        cycles += self.ppu.tick();
        cycles += self.cpu.tick();
        cycles
    }
}

impl Emulator {
    pub fn new(nes_file: &[u8], video_backend: impl VideoBackend + 'static) -> Self {

        let gamepack = GamePack::new(nes_file);

        let ppu_bus = PpuBus::new(gamepack.clone());
        let ppu = PPU::new(ppu_bus, video_backend);

        let apu = RAM::new(0x0018);
        let cpu_bus = CpuBus::new(ppu.clone(), apu, gamepack);

        let cpu = CPU::new(cpu_bus, 0x8000);

        log::info!("Emulator Created");

        Self { cpu, ppu }
    }
    pub fn reset(&mut self) {
        self.cpu.reset();
    }
}

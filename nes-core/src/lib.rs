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


#[test]
fn test() {

    use cpu6502::CPU;
    use ppu2c02::PPU;
    use motherboard::{cpubus::CpuBus, ppubus::PpuBus, ram::RAM};
    use gamepack::GamePack;

    let nes_file = [0u8; 100];
    let gamepack = GamePack::new(&nes_file);

    let ppu_bus = PpuBus::new(gamepack.get_ppu_half());
    let mut ppu = PPU::new(ppu_bus);

    let apu = RAM::new(0x0018);
    let cpu_bus = CpuBus::new(ppu.clone(), apu, gamepack);

    let mut cpu = CPU::new(cpu_bus, 0);

    loop {
        ppu.tick();
        ppu.tick();
        ppu.tick();
        cpu.tick();
    }
}
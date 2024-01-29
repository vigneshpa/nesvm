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
    use motherboard::{cpubus::CpuBus, ram::RAM};

    let ppu = RAM::new(0x0008);
    let apu = RAM::new(0x0018);
    let gamepack = RAM::new(0xBFE0);
    let bus = CpuBus::new(ppu, apu, gamepack);
    let mut cpu = CPU::new(bus, 0);

    loop {
        cpu.tick();
    }
}
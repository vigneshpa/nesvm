use nes_core::{cpu6502::CPU, motherboard::cpubus::CpuBus, Bus};

extern "C" {
    #[link_name = "ppu_read"]
    fn ffi_ppu_read(address: u16) -> u8;
    #[link_name = "apu_read"]
    fn ffi_apu_read(address: u16) -> u8;
    #[link_name = "gamepack_read"]
    fn ffi_gamepack_read(address: u16) -> u8;
    #[link_name = "ppu_write"]
    fn ffi_ppu_write(address: u16, data: u8) -> ();
    #[link_name = "apu_write"]
    fn ffi_apu_write(address: u16, data: u8) -> ();
    #[link_name = "gamepack_write"]
    fn ffi_gamepack_write(address: u16, data: u8) -> ();
}

pub fn ppu_read(address: u16) -> u8 {
    unsafe {
        ffi_ppu_read(address)
    }
}

pub fn apu_read(address: u16) -> u8 {
    unsafe {
        ffi_apu_read(address)
    }
}
pub fn gamepack_read(address: u16) -> u8 {
    unsafe {
        ffi_gamepack_read(address)
    }
}

pub fn ppu_write(address: u16, data: u8) -> () {
    unsafe {
        ffi_ppu_write(address, data)
    }
}

pub fn apu_write(address: u16, data: u8) -> () {
    unsafe {
        ffi_apu_write(address, data)
    }
}
pub fn gamepack_write(address: u16, data: u8) -> () {
    unsafe {
        ffi_gamepack_write(address, data)
    }
}

pub struct PPU {}

impl PPU {
    const fn new() -> Self {
        Self {}
    }
}

impl Bus for PPU {
    fn read(&self, address: u16) -> u8 {
        ppu_read(address)
    }

    fn write(&mut self, address: u16, data: u8) -> () {
        ppu_write(address, data)
    }
}

pub struct APU {}

impl APU {
    const fn new() -> Self {
        Self {}
    }
}

impl Bus for APU {
    fn read(&self, address: u16) -> u8 {
        apu_read(address)
    }

    fn write(&mut self, address: u16, data: u8) -> () {
        apu_write(address, data)
    }
}

pub struct GamePack {}

impl GamePack {
    const fn new() -> Self {
        Self {}
    }
}

impl Bus for GamePack {
    fn read(&self, address: u16) -> u8 {
        gamepack_read(address)
    }

    fn write(&mut self, address: u16, data: u8) -> () {
        gamepack_write(address, data)
    }
}

pub type FFICPU = CPU<CpuBus<APU, PPU, GamePack>>;

pub const fn construct() -> FFICPU {
    let apu = APU::new();
    let ppu = PPU::new();
    let gamepack = GamePack::new();
    let bus = CpuBus::new(apu, ppu, gamepack);
    let cpu = CPU::new(bus, 0x6000);
    return cpu;
}
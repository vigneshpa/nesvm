use crate::Bus;

mod parser;
mod m000;

pub use parser::parse as parse_nes_file;


pub enum NametableMirroring {
    Vertical,
    Horizontal,
}

pub enum System {
    NtscNES,
    LicensedPalNES,
    MultipleRegion,
    Dendy,
}
pub trait Mapper: Bus {
    fn ppu_read(&self, address: u16) -> u8;
    fn ppu_write(&mut self, address: u16, data: u8) -> ();
}
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

pub trait Mapper {
    fn ppu_bus(&mut self) -> &mut dyn Bus;
    fn cpu_bus(&mut self) -> &mut dyn Bus;
}

// pub trait Mapper<'a> {
//     type MapperCPU: Bus + 'a;
//     type MapperPPU: Bus + 'a;
//     fn cpu_bus(&'a mut self) -> Self::MapperCPU;
//     fn ppu_bus(&'a mut self) -> Self::MapperPPU;
// }
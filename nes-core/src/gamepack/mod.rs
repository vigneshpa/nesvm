pub mod ines;
pub mod mapper;
pub struct GamePack {
    pgr_rom: Vec<[u8; 0x4000]>,
    chr_rom: Vec<[u8; 0x2000]>,
}

trait Mapper {
    
}
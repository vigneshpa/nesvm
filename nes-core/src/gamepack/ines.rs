#![allow(dead_code)]

pub enum Mirroring {
    Horizontal,
    Vertical,
    FourScreen
}
pub struct INES {
    mapper: u16,
    battery: bool,
    mirroring: Mirroring,
    trainer: Option<[u8; 512]>,
    prg_rom: Vec<[u8; 16384]>,
    chr_rom: Vec<[u8; 8192]>,
    play_choice_inst_rom: Option<[u8; 8192]>,
    play_choice_prom: Option<[u8; 32]>,
}
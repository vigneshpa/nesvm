#![allow(dead_code)]
#![allow(unused)]
use crate::{gamepack::{GamePack, GamePackPPU}, Bus};

pub struct PpuBus {
    oam: Box<[u8; 512]>,
    memory: Box<[u8; 0x0800]>,
    pallete: Box<[u8; 0x0020]>,
    gamepack: GamePackPPU,
}

impl PpuBus {
    pub fn new(gamepack: GamePack) -> Self {
        Self {
            oam: Box::new([0u8; 512]),
            memory: Box::new([0u8; 0x0800]),
            pallete: Box::new([0u8; 0x0020]),
            gamepack: gamepack.get_ppu_half()
        }
    }
}

impl Bus for PpuBus {
    fn read(&self, address: u16) -> u8 {
        if address < 0x2000 {
            self.gamepack.read(address)
        } else {
            0
        }
    }

    fn write(&mut self, address: u16, data: u8) -> () {
        todo!()
    }
}
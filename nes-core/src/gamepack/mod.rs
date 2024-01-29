use std::{cell::RefCell, rc::Rc};

use crate::Bus;

use self::mapper::{decode_nes_file, Mapper};

pub mod mapper;
pub struct GamePack {
    mapper: Rc<RefCell<Box<dyn Mapper>>>
}

impl GamePack {
    pub fn new(nes_file:&[u8]) -> Self {
        Self {
            mapper: Rc::new(RefCell::new(decode_nes_file(nes_file)))
        }
    }
    pub fn get_ppu_half(&self) -> GamePackPPU {
        GamePackPPU {
            inner: self.clone()
        }
    }
}


impl Bus for GamePack {
    fn read(&self, address: u16) -> u8 {
        let mapper = self.mapper.borrow();
        mapper.read(address)
    }

    fn write(&mut self, address: u16, data: u8) -> () {
        let mut mapper = self.mapper.borrow_mut();
        mapper.write(address, data)
    }
}

impl Clone for GamePack {
    fn clone(&self) -> Self {
        Self { 
            mapper: self.mapper.clone()
        }
    }
}
pub struct GamePackPPU {
    inner: GamePack,
}

impl Bus for GamePackPPU {
    fn read(&self, address: u16) -> u8 {
        let mapper = self.inner.mapper.borrow();
        mapper.read(address)
    }

    fn write(&mut self, address: u16, data: u8) -> () {
        let mut mapper = self.inner.mapper.borrow_mut();
        mapper.write(address, data)
    }
}
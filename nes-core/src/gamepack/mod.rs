use self::mapper::{parse_nes_file, Mapper};
use crate::Bus;
use std::{cell::RefCell, ops::Deref, rc::Rc};

mod mapper;
pub use mapper::NametableMirroring;
pub use mapper::System;

pub struct GamePack {
    mapper: Rc<RefCell<Box<dyn Mapper>>>,
}

impl GamePack {
    pub fn new(nes_file: &[u8]) -> Self {
        Self {
            mapper: Rc::new(RefCell::new(parse_nes_file(nes_file).unwrap())),
        }
    }
    pub fn get_ppu_half(&self) -> GamePackPPU {
        GamePackPPU {
            inner: self.clone(),
        }
    }
}

impl Deref for GamePack {
    type Target = Rc<RefCell<Box<dyn Mapper>>>;

    fn deref(&self) -> &Self::Target {
        &self.mapper
    }
}

impl Bus for GamePack {
    fn read(&self, address: u16) -> u8 {
        self.borrow().read(address)
    }

    fn write(&mut self, address: u16, data: u8) -> () {
        self.borrow_mut().write(address, data)
    }
}

impl Clone for GamePack {
    fn clone(&self) -> Self {
        Self {
            mapper: self.mapper.clone(),
        }
    }
}
pub struct GamePackPPU {
    inner: GamePack,
}

impl Deref for GamePackPPU {
    type Target = GamePack;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Bus for GamePackPPU {
    fn read(&self, address: u16) -> u8 {
        self.borrow().ppu_read(address)
    }

    fn write(&mut self, address: u16, data: u8) -> () {
        self.borrow_mut().ppu_write(address, data)
    }
}

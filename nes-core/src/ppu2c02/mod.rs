mod ppucore;

use std::{cell::RefCell, rc::Rc};
use ppucore::Core;
use crate::{Bus, Tick};

pub struct PPU {
    inner: Rc<RefCell<Core>>
}

impl PPU {
    pub fn new() -> Self {
        Self {
            inner: Rc::new(RefCell::new(Core::new()))
        }
    }
}

impl Tick for PPU {
    fn tick(&mut self) -> u8 {
        0
    }
}

// PPU registers
impl Bus for PPU {
    fn read(&self, address: u16) -> u8 {
        let core = self.inner.borrow();
        core.read(address)
    }

    fn write(&mut self, address: u16, data: u8) {
        let mut core = self.inner.borrow_mut();
        core.write(address, data)
    }
}

impl Clone for PPU {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone()
        }
    }
}

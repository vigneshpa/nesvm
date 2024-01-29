mod core;

use std::{cell::RefCell, rc::Rc};
use core::Core;
use crate::{Bus, Tick};

pub struct PPU<B: Bus> {
    inner: Rc<RefCell<Core<B>>>
}

impl<B: Bus> PPU<B> {
    pub fn new(bus: B) -> Self {
        Self {
            inner: Rc::new(RefCell::new(Core::new(bus)))
        }
    }
}

impl<B: Bus> Tick for PPU<B> {
    fn tick(&mut self) -> u8 {
        let mut core = self.inner.borrow_mut();
        core.tick()
    }
}

// PPU registers
impl<B: Bus> Bus for PPU<B> {
    fn read(&self, address: u16) -> u8 {
        let core = self.inner.borrow();
        core.read(address)
    }

    fn write(&mut self, address: u16, data: u8) {
        let mut core = self.inner.borrow_mut();
        core.write(address, data)
    }
}

impl<B: Bus> Clone for PPU<B> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone()
        }
    }
}

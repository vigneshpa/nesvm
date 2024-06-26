mod core;

use crate::{Bus, Tick};
use core::Core;
use std::{cell::RefCell, ops::Deref, rc::Rc};


pub use self::core::VideoBackend;
pub use core::pixel::Pixel;

pub struct PPU<B: Bus> {
    inner: Rc<RefCell<Core<B>>>,
}

impl<B: Bus> PPU<B> {
    pub fn new(bus: B, video_backend: impl VideoBackend + 'static) -> Self {
        Self {
            inner: Rc::new(RefCell::new(Core::new(bus, video_backend))),
        }
    }
}

impl<B: Bus> Deref for PPU<B> {
    type Target = Rc<RefCell<Core<B>>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<B: Bus> Tick for PPU<B> {
    fn tick(&mut self) -> u8 {
        let mut core = self.borrow_mut();
        core.tick()
    }
}

// PPU registers
impl<B: Bus> Bus for PPU<B> {
    fn read(&self, address: u16) -> u8 {
        let core = self.borrow();
        core.read(address)
    }

    fn write(&mut self, address: u16, data: u8) {
        let mut core = self.borrow_mut();
        core.write(address, data)
    }
}

impl<B: Bus> Clone for PPU<B> {
    fn clone(&self) -> Self {
        Self{
            inner: self.inner.clone()
        }
    }
}

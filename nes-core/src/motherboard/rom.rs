use super::ram::RAM;
use crate::Bus;
use std::ops::Index;

/// A wrapper around RAM for Read Only Memory (ROM)
pub struct ROM {
    inner: RAM,
}

impl ROM {
    /// Create a ROM from RAM
    pub fn from_ram(inner: RAM) -> Self {
        ROM { inner }
    }
}

impl Bus for ROM {
    fn read(&self, address: u16) -> u8 {
        self.inner.read(address)
    }

    fn write(&mut self, _address: u16, _data: u8) -> () {}
}

impl Index<u16> for ROM {
    type Output = u8;
    fn index(&self, index: u16) -> &Self::Output {
        &self.inner[index]
    }
}

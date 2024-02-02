use super::ram::RAM;
use crate::Bus;
use std::ops::{Deref, DerefMut};

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

impl Deref for ROM {
    type Target = RAM;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for ROM {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Bus for ROM {
    fn read(&self, address: u16) -> u8 {
        self.inner.read(address)
    }

    fn write(&mut self, _address: u16, _data: u8) -> () {}
}

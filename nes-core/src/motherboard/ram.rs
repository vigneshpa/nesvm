use std::ops::{Deref, DerefMut};

use crate::Bus;

/// A container for Random Access Memory (RAM) that implements the `Bus` trait
pub struct RAM {
    inner: Box<[u8]>,
}

impl RAM {
    /// Create a new zero-initilized RAM with the given capacity
    pub fn new(capacity: usize) -> Self {
        let mem = vec![0u8; capacity];
        Self::from_slice(&mem)
    }
    /// Create a new RAM initilized with the given slice
    pub fn from_slice(slice: &[u8]) -> Self {
        if slice.len() > u16::MAX as usize + 1 {
            panic!(
                "Cannot allocate RAM with capacity {}, as the Bus has only 16-bit address lines",
                slice.len()
            );
        }
        Self {
            inner: Vec::from(slice).into_boxed_slice(),
        }
    }
}

impl Deref for RAM {
    type Target = Box<[u8]>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for RAM {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Bus for RAM {
    fn read(&self, address: u16) -> u8 {
        self[address as usize]
    }

    fn write(&mut self, address: u16, data: u8) {
        self[address as usize] = data;
    }
}

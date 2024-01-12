use std::ops::{IndexMut, Index};

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
            panic!("Cannot allocate RAM with capacity {}, as the Bus has only 16-bit address lines", slice.len());
        }
        Self {
            inner: Vec::from(slice).into_boxed_slice(),
        }
    }
}

impl Bus for RAM {
    fn get(&self, address: u16) -> u8 {
        self.inner[address as usize]
    }

    fn set(&mut self, address: u16, data: u8) {
        self.inner[address as usize] = data;
    }
}

impl IndexMut<u16> for RAM {
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        &mut self.inner[index as usize]
    }
}

impl Index<u16> for RAM {
    type Output = u8;
    fn index(&self, index: u16) -> &Self::Output {
        &self.inner[index as usize]
    }
}

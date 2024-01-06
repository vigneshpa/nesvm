use crate::cpu6502::Bus;

pub struct MotherBoard {
    memory:[u8; 65536]
}

impl MotherBoard {
    pub fn new() -> Self {
        Self {
            memory:[0u8; 65536]
        }
    }
}

impl Bus for MotherBoard {
    fn get(&self, address:u16) -> u8 {
        self.memory[address as usize]
    }

    fn set(&mut self, address:u16, data:u8) {
        self.memory[address as usize] = data;
    }
}
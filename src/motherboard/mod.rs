mod muxbus;
mod ram;

use self::{
    muxbus::{MuxBus, SubDevice},
    ram::RAM,
};
use crate::{cpu6502::CPU, Tick};

pub struct MotherBoardBuilder {
    system_bus: MuxBus,
}

/// A Bus de-multiplexor
impl MotherBoardBuilder {
    pub fn new() -> Self {
        Self {
            system_bus: MuxBus::new(),
        }
    }

    pub fn mount_memory(mut self, start: u16, end: u32, capacity: usize) -> Self {
        self.system_bus
            .mount(SubDevice::new(start, end, RAM::new(capacity)));
        self
    }

    pub fn get_board(self) -> MotherBoard {
        MotherBoard {
            cpu: CPU::new(self.system_bus),
        }
    }
}

pub struct MotherBoard {
    cpu: CPU<MuxBus>,
}

impl Tick for MotherBoard {
    fn tick(&mut self) {
        self.cpu.tick()
    }
}

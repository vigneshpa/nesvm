pub mod dynbus;

use self::dynbus::DynBus;
use crate::{cpu6502::CPU, Tick};

pub struct MotherBoard {
    cpu: CPU<DynBus>,
}

impl Tick for MotherBoard {
    fn tick(&mut self) {
        self.cpu.tick()
    }
}

mod addressing_mode;
mod cycles;
mod instruction;

use crate::Tick;

use self::{
    addressing_mode::{AddressingMode, Operand},
    instruction::Instruction,
};

pub trait Bus {
    fn get(&self, address: u16) -> u8;
    fn set(&mut self, address: u16, data: u8) -> ();
}

#[derive(Default)]
struct StatusRegister {
    carry: bool,
    zero: bool,
    disable_interrupts: bool,
    _decimal_mode: bool,
    break_: bool,
    _unused: bool,
    overflow: bool,
    negative: bool,
}

#[derive(Default)]
pub struct Registers {
    accumulator: u8,
    xindex: u8,
    yindex: u8,

    stack_pointer: u8,
    program_counter: u16,
    status_register: StatusRegister,
}

pub struct CPU<B: Bus> {
    // CPU State
    registers: Registers,
    pending_cycles: u8,
    // Functional parts
    bus: B,
}

impl<B: Bus> CPU<B> {
    pub fn new(bus: B) -> Self {
        Self {
            registers: Registers::default(),
            pending_cycles: 0,
            bus,
        }
    }

    pub fn from_state(registers:Registers, bus: B) -> Self {
        Self {
            registers,
            pending_cycles: 0,
            bus,
        }
    }

    pub fn set_pending_cycles(&mut self, cycles: u8) {
        self.pending_cycles = cycles;
    }

    pub fn read_next(&mut self) -> u8 {
        let next = self.bus.get(self.registers.program_counter);
        self.registers.program_counter = self.registers.program_counter.wrapping_add(1);
        next
    }

    pub fn read_next_u16(&mut self) -> u16 {
        let low = self.read_next();
        let high = self.read_next();
        crate::utils::concat(low, high)
    }

    pub fn load(&self, operand: Operand) -> u8 {
        operand.load(self)
    }

    pub fn store(&mut self, operand: Operand, data: u8) {
        operand.store(data, self);
    }
}

impl<B: Bus> Tick for CPU<B> {
    fn tick(&mut self) {
        //
        // Retrun if previous instructions have pending cycles
        if self.pending_cycles == 0 {
            //
            // Execute instruction
            let opcode = self.read_next();

            self.pending_cycles = cycles::required_for_opcode(opcode);

            let mode = AddressingMode::from_opcode(opcode);
            let instruction = Instruction::from_opcode(opcode);

            let operand = mode.fetch_operand(self);

            instruction.executor(self, operand).run();
        }

        self.pending_cycles -= 1;
    }
}

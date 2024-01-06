mod addressing_mode;
mod cycles;

use crate::Tickable;

use self::addressing_mode::AddressingMode;

pub trait Bus {
    fn get(&self, address: u16) -> u8;
    fn set(&mut self, address: u16, data: u8) -> ();
}

struct StatusRegister {
    carry: bool,
    zero: bool,
    disable_interrupts: bool,
    // decimal_mode: bool,
    break_: bool,
    // unused:bool,
    overflow: bool,
    negative: bool,
}

pub struct Registers {
    a: u8,
    x: u8,
    y: u8,

    sp: u8,
    pc: u16,
    st: StatusRegister,
}

pub struct CPU<B: Bus> {
    // CPU State
    registers: Registers,
    pending_cycles: u8,
    // Functional parts
    bus: B,
}

impl<B: Bus> CPU<B> {
    pub fn new(registers: Registers, bus: B) -> Self {
        Self {
            registers,
            pending_cycles: 0,
            bus,
        }
    }
    pub fn set_pending_cycles(&mut self, cycles: u8) {
        self.pending_cycles = cycles;
    }
}

impl<B: Bus> Tickable for CPU<B> {
    fn tick(&mut self) {
        //
        // Retrun if previous instructions have pending cycles
        if self.pending_cycles == 0 {
            //
            // Execute instruction
            let opcode = self.bus.get(self.registers.pc);
            self.registers.pc += 1;

            self.pending_cycles = cycles::required_for_opcode(opcode);

            let mode = AddressingMode::from_opcode(opcode);
            let operand = mode.fetch_operand(&mut self.registers, &mut self.bus);
        }

        self.pending_cycles -= 1;
    }
}

fn get_pocode_info(opcode: u8) -> () {}

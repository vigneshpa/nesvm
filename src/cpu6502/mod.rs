mod addressing_mode;
mod instruction;
mod opcode;

use crate::{Bus, Tick};

use self::{addressing_mode::Operand, opcode::Opcode};

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

impl StatusRegister {
    fn as_array(&self) -> [bool; 8] {
        [
            self.carry,
            self.zero,
            self.disable_interrupts,
            self._decimal_mode,
            self.break_,
            self._unused,
            self.overflow,
            self.negative,
        ]
    }

    fn from_array(data: [bool; 8]) -> Self {
        Self {
            carry: data[0],
            zero: data[1],
            disable_interrupts: data[2],
            _decimal_mode: data[3],
            break_: data[4],
            _unused: data[5],
            overflow: data[6],
            negative: data[7],
        }
    }

    fn set_array(&mut self, data: [bool; 8]) {
        self.carry = data[0];
        self.zero = data[1];
        self.disable_interrupts = data[2];
        self._decimal_mode = data[3];
        self.break_ = data[4];
        self._unused = data[5];
        self.overflow = data[6];
        self.negative = data[7];
    }

    fn get_u8(&self) -> u8 {
        let mut acc = 0u8;
        for el in self.as_array() {
            if el {
                acc |= 0b1000_0000u8;
            }
            acc = acc >> 1;
        }
        acc
    }

    fn set_u8(&mut self, mut data: u8) {
        let mut arr = [false; 8];
        for el in &mut arr {
            *el = (data & 0b1u8) != 0;
            data = data >> 1;
        }
        self.set_array(arr);
    }
}

struct Registers {
    accumulator: u8,
    xindex: u8,
    yindex: u8,

    stack_pointer: u8,
    program_counter: u16,
    status_register: StatusRegister,
}

/// A virtual 6502 CPU implementation
pub struct CPU<B: Bus> {
    // CPU State
    registers: Registers,
    pending_cycles: u8,
    // Functional parts
    bus: B,
}

impl<B: Bus> CPU<B> {

    /// Creates a new 6502 virtual CPU
    /// 
    /// * `bus` - The bus onto which the cpu is connected
    /// * `start_address` - Address at which the CPU must start executing instructions
    pub fn new(bus: B, start_address: u16) -> Self {
        Self {
            registers: Registers {
                accumulator:0,
                xindex:0,
                yindex:0,
                program_counter: start_address,
                stack_pointer: 0xfdu8,
                status_register: StatusRegister {
                    carry: false,
                    zero: false,
                    disable_interrupts: false,
                    _decimal_mode: false,
                    break_: false,
                    _unused: true,
                    overflow: false,
                    negative: false,
                },
            },
            pending_cycles: 0,
            bus,
        }
    }

    fn from_state(registers: Registers, bus: B) -> Self {
        Self {
            registers,
            pending_cycles: 0,
            bus,
        }
    }

    fn set_pending_cycles(&mut self, cycles: u8) {
        self.pending_cycles = cycles;
    }

    fn read_next(&mut self) -> u8 {
        let next = self.bus.get(self.registers.program_counter);
        self.registers.program_counter = self.registers.program_counter.wrapping_add(1);
        next
    }

    fn read_next_u16(&mut self) -> u16 {
        let low = self.read_next();
        let high = self.read_next();
        crate::utils::concat(low, high)
    }

    fn load(&self, operand: Operand) -> u8 {
        operand.load(self)
    }

    fn store(&mut self, operand: Operand, data: u8) {
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
            let opcode = Opcode::decode(opcode);

            self.pending_cycles = opcode.cycles;

            let operand = opcode.mode.fetch_operand(self);

            opcode.instruction.executor(self, operand).run();
        }

        self.pending_cycles -= 1;
    }
}

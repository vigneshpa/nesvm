mod addressing_mode;
mod instruction;
mod opcode;

use crate::utils;
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
    cycles: u8,
    // Functional parts
    pub bus: B,
    pending_irq: bool,
    pending_nmi: bool,
}

impl<B: Bus> Tick for CPU<B> {
    fn tick(&mut self) -> u8 {
        self.cycles = 0;

        if self.pending_nmi {
            self.handle_nmi();
            return self.cycles;
        }

        if self.pending_irq {
            self.handle_irq();
            return self.cycles;
        }

        // Execute instruction
        let opcode = self.read_next();
        let opcode = Opcode::decode(opcode);

        self.cycles += opcode.cycles;

        let operand = opcode.mode.fetch_operand(self);

        opcode.instruction.executor(self, operand).run();
        self.cycles
    }
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

    #[allow(unused)]
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

impl<B: Bus> CPU<B> {
    /// Creates a new 6502 virtual CPU
    ///
    /// * `bus` - The bus onto which the cpu is connected
    /// * `start_address` - Address at which the CPU must start executing instructions
    pub const fn new(bus: B, start_address: u16) -> Self {
        Self {
            registers: Registers {
                accumulator: 0,
                xindex: 0,
                yindex: 0,
                program_counter: start_address,
                stack_pointer: 0xFDu8,
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
            cycles: 0,
            pending_irq: false,
            pending_nmi: false,
            bus,
        }
    }

    fn read_next(&mut self) -> u8 {
        let next = self.bus.read(self.registers.program_counter);
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

    pub fn raise_irq(&mut self) {
        self.pending_irq = true;
    }

    fn handle_irq(&mut self) {
        if !self.registers.status_register.disable_interrupts {
            let (low, high) = utils::split(self.registers.program_counter);
            let status = self.registers.status_register.get_u8();
            self.push(high);
            self.push(low);
            self.push(status);
            self.registers.program_counter = self.read_vector(0xFFFE);
            self.registers.status_register.disable_interrupts = true;
        }
        self.pending_irq = false;
    }

    pub fn raise_nmi(&mut self) {
        self.pending_nmi = true;
    }

    fn handle_nmi(&mut self) {
        let (low, high) = utils::split(self.registers.program_counter);
        let status = self.registers.status_register.get_u8();
        self.push(high);
        self.push(low);
        self.push(status);
        self.registers.program_counter = self.read_vector(0xFFFA);
        self.registers.status_register.disable_interrupts = true;
        self.pending_nmi = false;
        self.pending_irq = false;
    }

    pub fn reset(&mut self) {
        self.registers.program_counter = self.read_vector(0xFFFC);
        self.cycles = 0;
    }

    fn read_vector(&self, base: u16) -> u16 {
        let low = self.bus.read(base);
        let high = self.bus.read(base + 1);
        utils::concat(low, high)
    }

    fn push(&mut self, data: u8) {
        let sp = self.get_sp();
        self.bus.write(sp as u16 | 0x100, data);
        self.set_sp(utils::decrement(sp)); // sp--
    }

    fn pull(&mut self) -> u8 {
        let sp = utils::increment(self.get_sp()); // sp++
        self.set_sp(sp);
        self.bus.read(sp as u16 | 0x100)
    }

    fn get_carry(&self) -> bool {
        self.registers.status_register.carry
    }

    fn set_carry(&mut self, val: bool) {
        self.registers.status_register.carry = val;
    }

    #[allow(unused)]
    fn get_overflow(&self) -> bool {
        self.registers.status_register.overflow
    }

    fn set_overflow(&mut self, val: bool) {
        self.registers.status_register.overflow = val;
    }

    fn get_acc(&self) -> u8 {
        self.registers.accumulator
    }

    fn set_acc(&mut self, data: u8) {
        self.registers.accumulator = data
    }

    fn get_x(&self) -> u8 {
        self.registers.xindex
    }

    fn set_x(&mut self, data: u8) {
        self.registers.xindex = data
    }

    fn get_y(&self) -> u8 {
        self.registers.yindex
    }

    fn set_y(&mut self, data: u8) {
        self.registers.yindex = data
    }

    fn get_sp(&self) -> u8 {
        self.registers.stack_pointer
    }

    fn set_sp(&mut self, data: u8) {
        self.registers.stack_pointer = data
    }

    fn set_nz(&mut self, data: u8) {
        self.registers.status_register.negative = data & 0x80 != 0;
        self.registers.status_register.zero = data == 0;
    }
}

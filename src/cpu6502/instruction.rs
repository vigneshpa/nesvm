use super::{addressing_mode::Operand, Bus, CPU};
use crate::utils::*;

/// Instructions for 6502
#[derive(Clone, Copy)]
pub enum Instruction {
    // Load and Store Instructions
    /// Load Accumulator with Memory
    LDA,
    /// Load Index X with Memory
    LDX,
    /// Load Index Y with Memory
    LDY,
    /// Store Accumulator in Memory
    STA,
    /// Store Index X in Memory
    STX,
    /// Store Index Y in Memory
    STY,

    // Arithmetic Operations
    /// Add Memory to Accumulator with Carry
    ADC,
    /// Subtract Memory from Accumulator with Borrow
    SBC,

    // Increment and Decrement
    /// Increment Memory by One
    INC,
    /// Increment Index X by One
    INX,
    /// Increment Index Y by One
    INY,

    /// Decrement Memory by One
    DEC,
    /// Decrement Index X by One
    DEX,
    /// Decrement Index Y by One
    DEY,

    // Shift and Rotate
    /// Arithmetic Shift Left One Bit
    ASL,
    /// Logical Shift Right One Bit
    LSR,

    /// Rotate Left One Bit
    ROL,
    /// Rotate Right One Bit
    ROR,

    // Logic
    /// AND Memory with Accumulator
    AND,
    /// OR Memory with Accumulator
    ORA,
    /// Exclusive-OR Memory with Accumulator
    EOR,

    // Compare and Test Bit
    /// Compare Memory and Accumulator
    CMP,
    /// Compare Memory and Index X
    CPX,
    /// Compare Memory with Index Y
    CPY,

    /// Test Bits in Memory with Accumulator
    BIT,

    // Branch
    /// Branch on Carry Clear
    BCC,
    /// Branch on Carry Set
    BCS,
    /// Branch on Result not Zero
    BNE,
    /// Branch on Result Zero
    BEQ,
    /// Branch on Result Plus
    BPL,
    /// Branch on Result Minus
    BMI,
    /// Branch on Overflow Clear
    BVC,
    /// Branch on Overflow Set
    BVS,

    // Transfer
    /// Transfer Accumulator to Index X
    TAX,
    /// Transfer Index X to Accumulator
    TXA,
    /// Transfer Accumulator to Index Y
    TAY,
    /// Transfer Index Y to Accumulator
    TYA,

    /// Transfer Stack Pointer to Index X
    TSX,
    /// Transfer Index X to Stack Pointer
    TXS,

    // Stack
    /// Push Accumulator on Stack
    PHA,
    /// Pull Accumulator from Stack
    PLA,
    /// Push Processor Status on Stack
    PHP,
    /// Pull Processor Status from Stack
    PLP,

    // Subroutines and Jump
    /// Jump to New Location
    JMP,
    /// Jump to New Location Saving Return Address
    JSR,
    /// Return from Subroutine
    RTS,
    /// Return from Interrupt
    RTI,

    // Set and Clear
    /// Clear Carry Flag
    CLC,
    /// Set Carry Flag
    SEC,
    /// Clear Decimal Mode
    CLD,
    /// Set Decimal Mode
    SED,
    /// Clear Interrupt Disable Status
    CLI,
    /// Set Interrupt Disable Status
    SEI,
    /// Clear Overflow Flag
    CLV,

    // Miscellaneous
    /// Break
    BRK,
    /// No Operation
    NOP,
}

impl Instruction {
    pub fn executor<'a, B: Bus>(
        self,
        cpu: &'a mut CPU<B>,
        operand: Operand,
    ) -> InstructionExecutor<'a, B> {
        InstructionExecutor::new(self, operand, cpu)
    }
}

pub struct InstructionExecutor<'a, B: Bus> {
    instruction: Instruction,
    operand: Operand,
    cpu: &'a mut CPU<B>,
}

impl<'a, B: Bus> InstructionExecutor<'a, B> {
    pub fn new(instruction: Instruction, operand: Operand, cpu: &'a mut CPU<B>) -> Self {
        Self {
            instruction,
            operand,
            cpu,
        }
    }

    pub fn run(&mut self) {
        use Instruction::*;
        match self.instruction {
            //
            // Load and Store
            LDA => {
                let data = self.load();
                self.set_nz(data);
                self.set_acc(data);
            }
            LDX => {
                let data = self.load();
                self.set_nz(data);
                self.set_x(data);
            }
            LDY => {
                let data = self.load();
                self.set_nz(data);
                self.set_y(data);
            }

            STA => self.store(self.get_acc()),
            STX => self.store(self.get_x()),
            STY => self.store(self.get_y()),

            // Addition and Subtraction
            ADC => {
                let a = self.get_acc() as u16;
                let b = self.load() as u16;
                let c = if self.get_carry() { 1 } else { 0 };
                let sum = a + b + c;
                self.set_carry((sum & 0x100) != 0);
                self.set_overflow(((a ^ sum) & (b ^ sum) & 0x80) != 0);
                let sum = sum as u8;
                self.set_nz(sum);
                self.set_acc(sum);
            }
            SBC => {
                let a = self.get_acc() as u16;
                let b = self.load() as u16;
                let c = if !self.get_carry() { 1 } else { 0 };
                let diff = a - b - c;
                self.set_carry(diff & 0x100 == 0);
                self.set_overflow(((a ^ diff) & (!b ^ diff) & 0x80) == 0);
                let diff = diff as u8;
                self.set_nz(diff);
                self.set_acc(diff);
            }

            // Increment
            INC => {
                let data = increment(self.load());
                self.set_nz(data);
                self.store(data);
            }
            INX => {
                let data = increment(self.get_x());
                self.set_nz(data);
                self.set_x(data);
            }
            INY => {
                let data = increment(self.get_y());
                self.set_nz(data);
                self.set_y(data);
            }

            // Decrement
            DEC => {
                let data = decrement(self.load());
                self.set_nz(data);
                self.store(data);
            }
            DEX => {
                let data = decrement(self.get_x());
                self.set_nz(data);
                self.set_x(data);
            }
            DEY => {
                let data = decrement(self.get_y());
                self.set_nz(data);
                self.set_y(data);
            }

            // Shift and Rotation
            ASL => {
                let val = self.load();
                let data = val << 1;
                self.set_carry(val & 0x80 != 0);
                self.set_nz(data);
                self.store(data);
            },
            LSR => {
                let val = self.load();
                let data = val >> 1;
                self.set_carry(val & 0x01 != 0);
                self.set_nz(data);
                self.store(data);
            },
            ROL => {
                let val = self.load();
                let mut data = val << 1;
                if self.get_carry() {
                    data |= 1;
                }
                self.set_carry(val & 0x80 != 0);
                self.set_nz(data);
                self.store(data);
            },
            ROR => {
                let val = self.load();
                let mut data = val >> 1;
                if self.get_carry() {
                    data |= 0x80;
                }
                self.set_carry(val & 0x01 != 0);
                self.set_nz(data);
                self.store(data);
            },

            // Bitwise
            AND => {
                let data = self.get_acc() & self.load();
                self.set_nz(data);
                self.set_acc(data);
            }
            ORA => {
                let data = self.get_acc() | self.load();
                self.set_nz(data);
                self.set_acc(data);
            }
            EOR => {
                let data = self.get_acc() ^ self.load();
                self.set_nz(data);
                self.set_acc(data);
            }

            // Compare
            CMP => self.compare(self.get_acc()),
            CPX => self.compare(self.get_x()),
            CPY => self.compare(self.get_y()),

            // Test bit
            BIT => {
                let data = self.load() & self.get_acc();
                self.set_nz(data);
                self.set_overflow(data & 0x40 != 0);
            }

            // Branching
            BCC => {
                if !self.cpu.registers.status_register.carry {
                    self.branch(self.load())
                }
            }
            BCS => {
                if self.cpu.registers.status_register.carry {
                    self.branch(self.load())
                }
            }
            BNE => {
                if !self.cpu.registers.status_register.zero {
                    self.branch(self.load())
                }
            }
            BEQ => {
                if self.cpu.registers.status_register.zero {
                    self.branch(self.load())
                }
            }
            BPL => {
                if !self.cpu.registers.status_register.negative {
                    self.branch(self.load())
                }
            }
            BMI => {
                if self.cpu.registers.status_register.negative {
                    self.branch(self.load())
                }
            }
            BVC => {
                if !self.cpu.registers.status_register.overflow {
                    self.branch(self.load())
                }
            }
            BVS => {
                if self.cpu.registers.status_register.overflow {
                    self.branch(self.load())
                }
            }

            // Transfer instructions
            TAX => {
                let data = self.get_acc();
                self.set_nz(data);
                self.set_x(data);
            }
            TXA => {
                let data = self.get_x();
                self.set_nz(data);
                self.set_acc(data);
            }
            TAY => {
                let data = self.get_acc();
                self.set_nz(data);
                self.set_y(data);
            }
            TYA => {
                let data = self.get_y();
                self.set_nz(data);
                self.set_acc(data);
            }
            TSX => {
                let data = self.get_sp();
                self.set_nz(data);
                self.set_x(data);
            }
            TXS => self.set_sp(self.get_x()),

            // Stack
            PHA => self.push(self.get_acc()),
            PLA => {
                let data = self.pull();
                self.set_nz(data);
                self.set_acc(data);
            }
            PHP => self.push(self.cpu.registers.status_register.get_u8()),
            PLP => {
                let data = self.pull();
                self.cpu.registers.status_register.set_u8(data)
            }

            // Jump

            // Jump to new location by changing the value of the program counter.
            JMP => if let Operand::Memory(memory) = self.operand {
                self.cpu.registers.program_counter = memory
            },

            // Jumps to a subroutine
            // The address before the next instruction (PC - 1) is pushed onto the stack: first the upper byte followed by the lower byte. As the stack grows backwards, the return address is therefore stored as a little-endian number in memory.
            // PC is set to the target address.
            JSR => if let Operand::Memory(memory) = self.operand {
                    let (low, high) = split(self.cpu.registers.program_counter - 1);
                    self.push(high);
                    self.push(low);
                    self.cpu.registers.program_counter = memory;
            },

            // Return from a subroutine to the point where it called with JSR.
            // The return address is popped from the stack (low byte first, then high byte).
            // The return address is incremented and stored in PC.
            RTS => {
                let low = self.pull();
                let high = self.pull();
                self.cpu.registers.program_counter = concat(low, high) + 1;
            }
            // Return from an interrupt.
            // P is popped from the stack.
            // PC is popped from the stack.
            RTI => {
                let status = self.pull();
                let low = self.pull();
                let high = self.pull();
                self.cpu.registers.status_register.set_u8(status);
                self.cpu.registers.program_counter = concat(low, high);
            }

            // Set and Clear
            CLC => self.cpu.registers.status_register.carry = false,
            SEC => self.cpu.registers.status_register.carry = true,
            CLD => self.cpu.registers.status_register._decimal_mode = false,
            SED => self.cpu.registers.status_register._decimal_mode = true,
            CLI => self.cpu.registers.status_register.disable_interrupts = false,
            SEI => self.cpu.registers.status_register.disable_interrupts = true,
            CLV => self.cpu.registers.status_register.overflow = false,

            // Force an Interrupt
            BRK => {
                let (low, high) = split(self.cpu.registers.program_counter);
                let status = self.cpu.registers.status_register.get_u8();
                self.push(high);
                self.push(low);
                self.push(status);
                self.cpu.registers.program_counter = self.read_irq_vector();
                self.cpu.registers.status_register.break_ = true;
            }

            // No Operation
            NOP => {}
        }
    }

    fn load(&self) -> u8 {
        self.cpu.load(self.operand)
    }

    fn store(&mut self, data: u8) {
        self.cpu.store(self.operand, data)
    }

    fn compare(&mut self, data: u8) {
        use std::cmp::Ordering::*;
        match data.cmp(&self.load()) {
            Less => {
                self.cpu.registers.status_register.negative = true;
                self.cpu.registers.status_register.zero = false;
                self.cpu.registers.status_register.carry = false;
            }
            Equal => {
                self.cpu.registers.status_register.negative = false;
                self.cpu.registers.status_register.zero = true;
                self.cpu.registers.status_register.carry = true;
            }
            Greater => {
                self.cpu.registers.status_register.negative = false;
                self.cpu.registers.status_register.zero = false;
                self.cpu.registers.status_register.carry = true;
            }
        }
    }

    fn branch(&mut self, data: u8) {
        let pc = self.cpu.registers.program_counter;
        self.cpu.registers.program_counter = signed_add(pc, data);
    }

    fn push(&mut self, data: u8) {
        let sp = self.get_sp();
        self.cpu.bus.set(sp as u16 | 0x100, data);
        self.set_sp(decrement(sp)); // sp--
    }

    fn pull(&mut self) -> u8 {
        let sp = increment(self.get_sp()); // sp++
        self.set_sp(sp);
        self.cpu.bus.get(sp as u16 | 0x100)
    }

    fn get_carry(&self) -> bool {
        self.cpu.registers.status_register.carry
    }

    fn set_carry(&mut self, val:bool) {
        self.cpu.registers.status_register.carry = val;
    }

    fn get_overflow(&self) -> bool {
        self.cpu.registers.status_register.overflow
    }

    fn set_overflow(&mut self, val:bool) {
        self.cpu.registers.status_register.overflow = val;
    }

    fn get_acc(&self) -> u8 {
        self.cpu.registers.accumulator
    }

    fn set_acc(&mut self, data: u8) {
        self.cpu.registers.accumulator = data
    }

    fn get_x(&self) -> u8 {
        self.cpu.registers.xindex
    }

    fn set_x(&mut self, data: u8) {
        self.cpu.registers.xindex = data
    }

    fn get_y(&self) -> u8 {
        self.cpu.registers.yindex
    }

    fn set_y(&mut self, data: u8) {
        self.cpu.registers.yindex = data
    }

    fn get_sp(&self) -> u8 {
        self.cpu.registers.stack_pointer
    }

    fn set_sp(&mut self, data: u8) {
        self.cpu.registers.stack_pointer = data
    }

    fn set_nz(&mut self, data: u8) {
        self.cpu.registers.status_register.negative = data & 0x80 != 0;
        self.cpu.registers.status_register.zero = data == 0;
    }

    fn read_irq_vector(&self) -> u16 {
        todo!()
    }
}

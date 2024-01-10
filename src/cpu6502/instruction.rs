use super::{addressing_mode::Operand, Bus, CPU, StatusRegister};
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
    pub fn from_opcode(_opcode: u8) -> Self {
        Self::NOP
    }

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
        use Instruction as I;
        match self.instruction {
            //
            // Load and Store
            I::LDA => self.set_acc(self.load()),
            I::LDX => self.set_x(self.load()),
            I::LDY => self.set_y(self.load()),

            I::STA => self.store(self.get_acc()),
            I::STX => self.store(self.get_x()),
            I::STY => self.store(self.get_y()),

            // Addition and Subtraction
            I::ADC => self.set_acc(add_with_carry(
                self.get_acc(),
                self.load(),
                self.get_carry(),
            )),
            I::SBC => self.set_acc(sub_with_carry(
                self.get_acc(),
                self.load(),
                self.get_carry(),
            )),

            // Increment
            I::INC => self.store(increment(self.load())),
            I::INX => self.set_x(increment(self.get_x())),
            I::INY => self.set_y(increment(self.get_y())),

            // Decrement
            I::DEC => self.store(decrement(self.load())),
            I::DEX => self.set_x(decrement(self.get_x())),
            I::DEY => self.set_y(decrement(self.get_y())),

            // Shift and Rotation
            I::ASL => self.store(shift_left(self.load())),
            I::LSR => self.store(shift_right(self.load())),
            I::ROL => self.store(rotate_left(self.load())),
            I::ROR => self.store(rotate_right(self.load())),

            // Bitwise
            I::AND => self.set_acc(self.get_acc() & self.load()),
            I::ORA => self.set_acc(self.get_acc() | self.load()),
            I::EOR => self.set_acc(self.get_acc() ^ self.load()),

            // Compare
            I::CMP => self.compare(self.get_acc()),
            I::CPX => self.compare(self.get_x()),
            I::CPY => self.compare(self.get_y()),

            // Test bit
            I::BIT => todo!(),

            // Branching
            I::BCC => {
                if !self.cpu.registers.status_register.carry {
                    self.branch(self.load())
                }
            },
            I::BCS => {
                if self.cpu.registers.status_register.carry {
                    self.branch(self.load())
                }
            },
            I::BNE => {
                if !self.cpu.registers.status_register.zero {
                    self.branch(self.load())
                }
            },
            I::BEQ => {
                if self.cpu.registers.status_register.zero {
                    self.branch(self.load())
                }
            },
            I::BPL => {
                if !self.cpu.registers.status_register.negative {
                    self.branch(self.load())
                }
            },
            I::BMI => {
                if self.cpu.registers.status_register.negative {
                    self.branch(self.load())
                }
            },
            I::BVC => {
                if !self.cpu.registers.status_register.overflow {
                    self.branch(self.load())
                }
            },
            I::BVS => {
                if self.cpu.registers.status_register.overflow {
                    self.branch(self.load())
                }
            },

            // Transfer instructions
            I::TAX => self.set_x(self.get_acc()),
            I::TXA => self.set_acc(self.get_x()),
            I::TAY => self.set_y(self.get_acc()),
            I::TYA => self.set_acc(self.get_y()),
            I::TSX => self.set_x(self.get_sp()),
            I::TXS => self.set_sp(self.get_x()),

            // Stack
            I::PHA => self.push(self.get_acc()),
            I::PLA => {
                let data = self.pull();
                self.set_acc(data)
            },
            I::PHP => self.push(self.get_status()),
            I::PLP => {
                let data = self.pull();
                self.set_status(data)
            },

            // Jump
            I::JMP => match self.operand {
                Operand::Memory(memory) => self.cpu.registers.program_counter = memory,
                _ => panic!("Illeagal addressing mode")
            },
            I::JSR => match self.operand {
                Operand::Memory(memory) => {
                    self.cpu.registers.program_counter = memory;
                    todo!("Push the address before the next instruction (PC-1)");
                },
                _ => panic!("Illeagal addressing mode")
            },
            I::RTI => todo!(),
            I::CLC => todo!(),
            I::SEC => todo!(),
            I::CLD => todo!(),
            I::SED => todo!(),
            I::CLI => todo!(),
            I::SEI => todo!(),
            I::CLV => todo!(),
            I::BRK => todo!(),
            I::NOP => {}
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
        self.cpu.bus.set(sp as u16 + 0b01_00u16, data);
        self.set_sp(decrement(sp)); // sp--
    }

    fn pull(&mut self) -> u8 {
        let sp = increment(self.get_sp());
        self.set_sp(sp); // sp++
        self.cpu.bus.get(sp as u16 + 0b01_00u16)
    }

    fn get_status(&self) -> u8 {
        let mut acc = 0u8;
        for el in self.cpu.registers.status_register.as_array() {
            if el {
                acc |= 1u8;
            }
            acc = acc << 1;
        }
        acc
    }

    fn set_status(&mut self, mut data: u8) {
        let mut arr = [false; 8];
        let high_mask = 0b1000_0000u8;
        for el in &mut arr {
            *el = (high_mask & data) != 0;
            data = data << 1;
        }
        self.cpu.registers.status_register = StatusRegister::from_array(arr);
    }

    fn get_carry(&self) -> bool {
        self.cpu.registers.status_register.carry
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
}

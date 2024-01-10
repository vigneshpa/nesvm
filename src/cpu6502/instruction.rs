use super::{addressing_mode::Operand, Bus, CPU};
use crate::utils::*;

/// Instructions for 6502
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
    pub fn from_opcode(opcode: u8) -> Self {
        Self::LDA
    }

    pub fn executor<'a, B: Bus>(self, cpu: &'a mut CPU<B>, operand: Operand) -> InstructionExecuter<'a, B> {
        InstructionExecuter::new(self, operand, cpu)
    }
}

pub struct InstructionExecuter<'a, B: Bus> {
    instruction: Instruction,
    operand: Operand,
    cpu: &'a mut CPU<B>,
}

impl<'a, B: Bus> InstructionExecuter<'a, B> {

    pub fn new(instruction:Instruction, operand: Operand, cpu: &'a mut CPU<B>) -> Self {
        Self {
            instruction,
            operand,
            cpu
        }
    }

    fn load(&self) -> u8 {
        self.cpu.load(self.operand)
    }

    fn store(&mut self, data: u8) {
        self.cpu.store(self.operand, data)
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

    fn get_carry(&self) -> bool {
        self.cpu.registers.status_register.carry
    }

    pub fn run(&mut self) {
        use Instruction as I;
        match self.instruction {
            I::LDA => self.set_acc(self.load()),
            I::LDX => self.set_x(self.load()),
            I::LDY => self.set_y(self.load()),
            I::STA => self.store(self.get_acc()),
            I::STX => self.store(self.get_x()),
            I::STY => self.store(self.get_y()),
            I::ADC => self.set_acc(add_with_carry(
                self.get_acc(),
                self.load(),
                self.get_carry(),
            )),
            I::SBC => self.set_acc(add_with_carry(
                self.get_acc(),
                self.load(),
                self.get_carry(),
            )),
            I::INC => self.store(increment(self.load())),
            I::INX => self.set_x(increment(self.get_x())),
            I::INY => self.set_y(increment(self.get_y())),
            I::DEC => self.store(decrement(self.load())),
            I::DEX => self.set_x(decrement(self.get_x())),
            I::DEY => self.set_y(decrement(self.get_y())),
            I::ASL => self.store(shift_left(self.load())),
            I::LSR => self.store(shift_right(self.load())),
            I::ROL => self.store(rotate_left(self.load())),
            I::ROR => self.store(rotate_right(self.load())),
            I::AND => self.set_acc(self.get_acc() & self.load()),
            I::ORA => todo!(),
            I::EOR => todo!(),
            I::CMP => todo!(),
            I::CPX => todo!(),
            I::CPY => todo!(),
            I::BIT => todo!(),
            I::BCC => todo!(),
            I::BCS => todo!(),
            I::BNE => todo!(),
            I::BEQ => todo!(),
            I::BPL => todo!(),
            I::BMI => todo!(),
            I::BVC => todo!(),
            I::BVS => todo!(),
            I::TAX => todo!(),
            I::TXA => todo!(),
            I::TAY => todo!(),
            I::TYA => todo!(),
            I::TSX => todo!(),
            I::TXS => todo!(),
            I::PHA => todo!(),
            I::PLA => todo!(),
            I::JMP => todo!(),
            I::JSR => todo!(),
            I::RTI => todo!(),
            I::CLC => todo!(),
            I::SEC => todo!(),
            I::CLD => todo!(),
            I::SED => todo!(),
            I::CLI => todo!(),
            I::SEI => todo!(),
            I::CLV => todo!(),
            I::BRK => todo!(),
            I::NOP => todo!(),
            // Arithmetic Operations
        }
    }
}

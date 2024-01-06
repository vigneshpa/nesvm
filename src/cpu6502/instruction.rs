use super::{addressing_mode::Operand, Bus, Registers};

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

    pub fn run(&self, operand: Operand, registers: &mut Registers, bus: &mut impl Bus) {
        match self {
            Self::LDA => registers.accumulator = operand.load(registers, bus),
            Self::LDX => registers.xindex = operand.load(registers, bus),
            Self::LDY => registers.yindex = operand.load(registers, bus),
            Self::STA => operand.store(registers.accumulator, registers, bus),
            Self::STX => operand.store(registers.xindex, registers, bus),
            Self::STY => operand.store(registers.yindex, registers, bus),
            Self::ADC => todo!(),
            Self::SBC => todo!(),
            Self::INC => todo!(),
            Self::INX => todo!(),
            Self::INY => todo!(),
            Self::DEC => todo!(),
            Self::DEX => todo!(),
            Self::DEY => todo!(),
            Self::ASL => todo!(),
            Self::LSR => todo!(),
            Self::ROL => todo!(),
            Self::ROR => todo!(),
            Self::AND => todo!(),
            Self::ORA => todo!(),
            Self::EOR => todo!(),
            Self::CMP => todo!(),
            Self::CPX => todo!(),
            Self::CPY => todo!(),
            Self::BIT => todo!(),
            Self::BCC => todo!(),
            Self::BCS => todo!(),
            Self::BNE => todo!(),
            Self::BEQ => todo!(),
            Self::BPL => todo!(),
            Self::BMI => todo!(),
            Self::BVC => todo!(),
            Self::BVS => todo!(),
            Self::TAX => todo!(),
            Self::TXA => todo!(),
            Self::TAY => todo!(),
            Self::TYA => todo!(),
            Self::TSX => todo!(),
            Self::TXS => todo!(),
            Self::PHA => todo!(),
            Self::PLA => todo!(),
            Self::JMP => todo!(),
            Self::JSR => todo!(),
            Self::RTI => todo!(),
            Self::CLC => todo!(),
            Self::SEC => todo!(),
            Self::CLD => todo!(),
            Self::SED => todo!(),
            Self::CLI => todo!(),
            Self::SEI => todo!(),
            Self::CLV => todo!(),
            Self::BRK => todo!(),
            Self::NOP => todo!(),

            // Arithmetic Operations

        }
    }
}

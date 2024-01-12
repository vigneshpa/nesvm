use super::{addressing_mode::AddressingMode, instruction::Instruction};

pub struct Opcode {
    pub instruction: Instruction,
    pub mode: AddressingMode,
    pub cycles: u8,
}

impl Opcode {
    pub fn decode(code: u8) -> Opcode {
        use Instruction::*;
        use AddressingMode::*;

         // Generated from the build script
        let opcode = include!(concat!(env!("OUT_DIR"), "/opcode_match.rs"));
        opcode
    }
}
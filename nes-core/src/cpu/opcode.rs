use super::{addressing_mode::AddressingMode, instruction::Instruction};

pub struct Opcode {
    pub instruction: Instruction,
    pub mode: AddressingMode,
    pub cycles: u8,
}

impl Opcode {
    pub fn decode(code: u8) -> Opcode {
        use AddressingMode::*;
        use Instruction::*;

        // Decoder is generated from the build script
        include!(concat!(env!("OUT_DIR"), "/opcode_match.rs"))
    }
}

use super::{addressing_mode::AddressingMode, instruction::Instruction};

struct Opcode {
    instruction: Instruction,
    mode: AddressingMode,
    cycles: u8,
}

impl Opcode {
    pub fn decode(code: u8) -> Opcode {
        use Instruction::*;
        use AddressingMode::*;

        // Match
        match code {
            _ => todo!("Unknown code")
        }
    }
}
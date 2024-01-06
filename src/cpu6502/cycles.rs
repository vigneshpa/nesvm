const LOOKUP_TABLE: [u8; 256] = [0u8; 256];
pub fn required_for_opcode(opcode: u8) -> u8 {
    LOOKUP_TABLE[opcode as usize]
}

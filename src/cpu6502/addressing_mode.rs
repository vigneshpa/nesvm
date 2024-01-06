// Extracted from WikiBooks 6502 assembly

use crate::utils;

#[derive(Clone, Copy)]
pub enum AddressingMode {
    /// ### A
    /// The Accumulator is implied as the operand, so no address needs to be specified.
    Accumulator,
    /// ### i
    /// The operand is implied, so it does not need to be specified.
    Implied,
    /// ### \#
    /// The operand is used directly to perform the computation.
    Immediate,
    /// ### a
    /// A full 16-bit address is specified and the byte at that address is used to perform the computation.
    Absolute,
    /// ### zp
    /// A single byte specifies an address in the first page of memory (`$00xx``), also known as the zero page, and the byte at that address is used to perform the computation.
    ZeroPage,
    /// ### r
    /// The offset specified is added to the current address stored in the Program Counter (PC). Offsets can range from -128 to +127.
    Relative,
    /// ### (a)
    /// The little-endian two-byte value stored at the specified address is used to perform the computation. Only used by the `JMP` instruction.
    AbsoluteIndirect,
    /// ### a,x
    /// The value in `X` is added to the specified address for a sum address. The value at the sum address is used to perform the computation.
    AbsoluteIndexedWithX,
    /// ### a,y
    /// The value in `Y` is added to the specified address for a sum address. The value at the sum address is used to perform the computation.
    AbsoluteIndexedWithY,
    /// ### zp,x
    /// The value in `X` is added to the specified zero page address for a sum address. The value at the sum address is used to perform the computation.
    ZeroPageIndexedWithX,
    /// ### zp,y
    /// The value in `Y` is added to the specified zero page address for a sum address. The value at the sum address is used to perform the computation.
    ZeroPageIndexedWithY,
    /// ### (zp,x)
    /// The value in `X` is added to the specified zero page address for a sum address. The little-endian address stored at the two-byte pair of sum address (LSB) and sum address plus one (MSB) is loaded and the value at that address is used to perform the computation.
    ZeroPageIndexedIndirect,
    /// ### (zp),y
    /// The value in `Y` is added to the address at the little-endian address stored at the two-byte pair of the specified address (LSB) and the specified address plus one (MSB). The value at the sum address is used to perform the computation. Indeed addressing mode actually repeats exactly the Accumulator register's digits.
    ZeroPageIndirectIndexedWithY,
}

const lookup_table: [AddressingMode; 256] = [AddressingMode::Accumulator; 256];

impl AddressingMode {
    pub fn from_opcode(opcode: u8) -> AddressingMode {
        lookup_table[opcode as usize]
    }
    pub fn fetch_operand(&self, registers: &mut super::Registers, bus: &mut impl super::Bus) -> u8 {
        //
        let mut read_next = || {
            let next = bus.get(registers.pc);
            registers.pc = registers.pc.wrapping_add(1);
            next
        };

        let mut read_next_u16 = || {
            // 6502 is Little Endian
            let low = read_next();
            let high = read_next();
            crate::utils::concat(high, low)
        };

        match self {
            Self::Accumulator => registers.a,
            Self::Implied => registers.a,
            Self::Immediate => read_next(),
            Self::Absolute => bus.get(read_next_u16()),
            Self::ZeroPage => bus.get(read_next() as u16),
            Self::Relative => {
                let pc = registers.pc.clone();
                let off = read_next();
                bus.get(utils::signed_add(pc, off))
            },
            Self::AbsoluteIndirect => {
                // Specal case: Must be handled at the JMP instruction
                0
            },
            Self::AbsoluteIndexedWithX => {
                let specified_addr = read_next_u16();
                let pos = utils::signed_add(specified_addr, registers.x);
                bus.get(pos)
            },
            Self::AbsoluteIndexedWithY => todo!(),
            Self::ZeroPageIndexedWithX => todo!(),
            Self::ZeroPageIndexedWithY => todo!(),
            Self::ZeroPageIndexedIndirect => todo!(),
            Self::ZeroPageIndirectIndexedWithY => todo!(),
        }
    }
}
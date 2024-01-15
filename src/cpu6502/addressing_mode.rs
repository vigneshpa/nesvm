// Extracted from WikiBooks 6502 assembly

use super::{Bus, CPU};
use crate::utils::{self, concat, increment};

/// Addressing modes for 6052
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
    AbsoluteIndirect, // Not for this implementation

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

#[derive(Clone, Copy)]
pub enum Operand {
    Implied,
    Accumulator,
    Memory(u16),
}

impl Operand {
    pub fn load<B: Bus>(&self, cpu: &CPU<B>) -> u8 {
        match self {
            Self::Implied => 0,
            Self::Accumulator => cpu.registers.accumulator,
            Self::Memory(pointer) => cpu.bus.read(*pointer),
        }
    }
    pub fn store<B: Bus>(&self, data: u8, cpu: &mut CPU<B>) {
        match self {
            Self::Implied => (),
            Self::Accumulator => cpu.registers.accumulator = data,
            Self::Memory(pointer) => cpu.bus.write(*pointer, data),
        }
    }
}

impl AddressingMode {
    pub fn fetch_operand<B: Bus>(&self, cpu: &mut CPU<B>) -> Operand {
        //
        let pc = cpu.registers.program_counter;

        match self {
            Self::Accumulator => Operand::Accumulator,
            Self::Implied => Operand::Implied,
            Self::Immediate => {
                cpu.registers.program_counter += pc.wrapping_add(1);
                Operand::Memory(pc)
            }
            Self::Absolute => Operand::Memory(cpu.read_next_u16()),
            Self::ZeroPage => Operand::Memory(cpu.read_next() as u16),
            Self::Relative => Operand::Memory(utils::signed_add(pc, cpu.read_next())),
            // Specal case for JMP instruction
            Self::AbsoluteIndirect => Operand::Memory({
                let low = cpu.read_next();
                let high = cpu.read_next();
                let low = cpu.bus.read(concat(low, high));
                let high = cpu.bus.read(concat(low, increment(high)));
                let pointer = concat(low, high);
                pointer
            }),
            Self::AbsoluteIndexedWithX => {
                Operand::Memory(cpu.read_next_u16() + cpu.registers.xindex as u16)
            }
            Self::AbsoluteIndexedWithY => {
                Operand::Memory(cpu.read_next_u16() + cpu.registers.yindex as u16)
            }
            Self::ZeroPageIndexedWithX => {
                Operand::Memory(cpu.read_next() as u16 + cpu.registers.xindex as u16)
            }
            Self::ZeroPageIndexedWithY => {
                Operand::Memory(cpu.read_next() as u16 + cpu.registers.yindex as u16)
            }
            Self::ZeroPageIndexedIndirect => {
                let sum_address = cpu.read_next() as u16 + cpu.registers.xindex as u16;
                let low = cpu.bus.read(sum_address);
                let high = cpu.bus.read(sum_address + 1);
                let pointer = utils::concat(low, high);
                Operand::Memory(pointer)
            }
            Self::ZeroPageIndirectIndexedWithY => {
                let off = cpu.read_next() as u16;
                let low = cpu.bus.read(off);
                let high = cpu.bus.read(off + 1);
                let pointer = utils::concat(low, high);
                Operand::Memory(pointer)
            }
        }
    }
}

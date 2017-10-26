//! MIPS instructions

use super::{FunctionCode, OpCode};

/// Fetch function code bits
fn function_bits(v: &u32) -> u8
{
    (0b111111 & v) as u8
}

/// Fetch shift bits
fn shift_bits(v: &u32) -> u8
{
    let mask = 0b11111 << 6;
    ((v & mask) >> 6) as u8
}

/// Fetch bits opcode bits
fn opcode_bits(v: &u32) -> u8
{
    let mask = 0b11111 << 26;
    ((v & mask) >> 26) as u8
}

/// Fetch jump offset bits
fn offset_bits(v: &u32) -> u32
{
    //  I swear this is 26 bits
    v & 0b11111111111111111111111111
}

/// Fetch immediate value bits
fn immediate_bits(v: &u32) -> u16
{
    (v & 0b1111111111111111) as u16
}

/// Extract source 1 bits
fn src1_bits(v: &u32) -> u8
{
    let mask = 0b11111 << 21;
    ((v & mask) >> 21) as u8
}

/// Extract source 2 bits
fn src2_bits(v: &u32) -> u8
{
    let mask = 0b11111 << 16;
    ((v & mask) >> 16) as u8
}

/// Extract destination bits
fn destination_bits(v: &u32) -> u8
{
    let mask = 0b11111 << 11;
    ((v & mask) >> 11) as u8
}

/// A 32-bit MIPS instruction
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Instruction32 {
    /// A Register type instruction
    ///
    /// ## Layout
    ///
    /// **Note**: intervals are closed
    ///
    /// | OpCode | Source 1 | Source 2 | Destination | Shift amount | Function |
    /// |--------|----------|----------|-------------|--------------|----------|
    /// | 000000 | 5 bits   | 5 bits   | 5 bits      | 5 bits       | 6 bits   |
    /// | 26..31 | 21..25   | 16..20   | 11..15      | 6..10        | 0..5     |
    R {
        /// Source 1
        src1: u8,
        /// Source 2
        src2: u8,
        /// Destination
        dst: u8,
        /// Shift about
        shift: u8,
        /// Function code
        func: FunctionCode,
    },

    /// An Immediate instruction (32-bit)
    ///
    /// ## Layout
    ///
    /// **Note**: intervals are closed
    ///
    /// | OpCode | Source | Destination | Immediate Value |
    /// |--------|--------|-------------|-----------------|
    /// | 6 bits | 5 bits | 5 bits      | 16 bits         |
    /// | 26..31 | 21..25 | 16..20      | 0..15           |
    ///
    I {
        /// OpCode
        opcode: OpCode,
        /// Source
        src: u8,
        /// Destination
        dst: u8,
        /// Immediate value
        imm: u16,
    },

    /// A Jump type instruction, used for jumps
    ///
    /// ## Layout
    ///
    /// **Note**: intervals are closed
    ///
    /// | OpCode | Target offset |
    /// |--------|---------------|
    /// | 0001xx | 26 bits       |
    /// | 26..31 | 0..25         |
    ///
    J {
        /// OpCode
        opcode: OpCode,
        /// Offset from pc
        offset: u32,
    },

    /// A Coprocessor instruction
    C {
        // TODO(will): c-type instruction fields
    },
}

impl From<u32> for Instruction32 {
    fn from(v: u32) -> Instruction32
    {
        let opcode_u8 = opcode_bits(&v);

        // R-type
        if opcode_u8 == 0b00000
        {
            Instruction32::R {
                src1: src1_bits(&v),
                src2: src2_bits(&v),
                dst: destination_bits(&v),
                shift: shift_bits(&v),
                func: FunctionCode::from(function_bits(&v)),
            }
        }
        // J-type
        else if opcode_u8 >= 0b00100
        {
            Instruction32::J {
                opcode: OpCode::from(opcode_u8),
                offset: offset_bits(&v),
            }
        }
        // I-type
        else
        {
            Instruction32::I {
                opcode: OpCode::from(opcode_u8),
                src: src1_bits(&v),
                dst: src2_bits(&v),
                imm: immediate_bits(&v),
            }
        }
    }
}


#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn decode_r_instruction()
    {
        //              oooooo1111122222dddddsssssffffff
        let encoded = 0b00000000011001010100101111100000;

        let expected = Instruction32::R {
            src1: 3,
            src2: 5,
            dst: 9,
            shift: 15,
            func: FunctionCode::add
        };
        
        assert_eq!(expected, Instruction32::from(encoded));
    }
}

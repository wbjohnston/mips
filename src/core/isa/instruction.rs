//! MIPS instructions

use super::{FunctionCode, OpCode};

/// A 32-bit MIPS instruction
pub enum Instruction32
{
    /// A Register type instruction
    ///
    /// ## Layout
    /// | OpCode | Source 1 | Source 2 | Destination | Shift amount | Function |
    /// |--------|----------|----------|-------------|--------------|----------|
    /// | 000000 | 5 bits   | 5 bits   | 5 bits      | 5 bits       | 6 bits   |
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
        func: FunctionCode
    },
    
    /// An Immediate instruction (32-bit)
    ///
    /// ## Layout
    /// | OpCode | Source | Destination | Immediate Value |
    /// |--------|--------|-------------|-----------------|
    /// | 6 bits | 5 bits | 5 bits      | 16 bits         |
    ///
    I {
        /// OpCode
        opcode: OpCode,
        /// Source
        src: u8,
        /// Destination
        dst: u8,
        /// Immediate value
        imm: u16
    },


    /// A Jump type instruction, used for jumps
    /// 
    /// ## Layout
    /// | OpCode | Target offset |
    /// |--------|---------------|
    /// | 0001xx | 26 bits       |
    ///
    J {
        /// OpCode
        opcode: OpCode,
        /// Offset from pc
        offset: u32
    },

    /// A Coprocessor instruction
    C {
        // TODO(will): c-type instruction fields
    }
}

impl From<u32> for Instruction32
{
    fn from(v: u32) -> Self
    {
        unimplemented!();
    }
}

//! MIPS Function codes (used in R-type instructions when opcode is 0b00000) 

/// R type function code
#[repr(u8)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum FunctionCode {
    /// Add
    add = 0x20,

    /// Add (unsigned)
    addu = 0x21,

    /// Bitwise and
    and = 0x24,

    /// Divide
    div = 0x1A,

    /// Divide (unsigned)
    divu = 0x1B, // Divide (unsigned)

    /// Move from high
    mfhi = 0x10,

    /// Move from low
    mflo = 0x12,

    // TODO(will): implement Coprocessor instruction
    // mfc0 = 0x, // Move from co-processor 0
    /// Multiply
    mult = 0x18,

    /// Multiply (unsigned)
    multu = 0x19,

    /// Nor
    nor = 0x27,

    /// Bitwise Exclusive Or
    xor = 0x26,

    /// Bitwise Or
    or = 0x25,

    /// Set to 1 if less than
    slt = 0x2A,

    /// Set to 1 if less than (unsigned)
    sltu = 0x2B,

    /// Logical Shift left
    sll = 0x00,

    /// Logical Shift right (0-extended)
    srl = 0x02,

    /// Arithmetic Shift Right (sign-extended)
    sra = 0x03,

    /// Subtract
    sub = 0x22,

    /// Subtract (unsigned)
    subu = 0x23,
}

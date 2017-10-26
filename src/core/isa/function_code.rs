//! MIPS Function codes (used in R-type instructions when opcode is 0b00000)

/// R type function code
#[repr(u8)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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


impl From<u8> for FunctionCode {
    #[allow(unused_variables)]
    fn from(v: u8) -> Self
    {
        use self::FunctionCode::*;
        match v
        {
            0x20 => add,

            0x21 => addu,

            0x24 => and,

            0x1A => div,

            0x1B => divu,

            0x10 => mfhi,

            0x12 => mflo,

            0x18 => mult,

            0x19 => multu,

            0x27 => nor,

            0x26 => xor,

            0x25 => or,

            0x2A => slt,

            0x2B => sltu,

            0x00 => sll,

            0x02 => srl,

            0x03 => sra,

            0x22 => sub,

            0x23 => subu,
            // TODO(will): implement Coprocessor instruction
            e => panic!("{} is not an opcode."),
        }
    }
}

#[cfg(test)]
mod test
{


}

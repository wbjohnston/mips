//! Mips Opcodes

/// A MIPS opcode
#[repr(u8)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum OpCode {
    ////////////////////////////////////////////////////////////////////////////
    // I-type instructions
    ////////////////////////////////////////////////////////////////////////////
    /// Add (immediate)
    addi = 0x08,

    /// Add (immediate, unsigned)
    addiu = 0x09,

    /// Bitwise and (immediate)
    andi = 0x0C,

    /// Branch if equal
    beq = 0x04,

    /// Branch if not equal
    bne = 0x05,

    /// Load byte (unsigned)
    lbu = 0x24,

    /// Load half-word (unsigned)
    lhu = 0x025,

    /// Load upper (immediate)
    lui = 0x0F,

    /// Load word
    lw = 0x23,

    /// Bitwise or (immediate)
    ori = 0x0D,

    /// Store byte
    sb = 0x28,

    /// Store half-word
    sh = 0x29,

    /// Set to 1 if less than (immediate)
    slti = 0x0A,

    /// Store word
    sw = 0x2B,
    ////////////////////////////////////////////////////////////////////////////
    // J-type instructions
    ////////////////////////////////////////////////////////////////////////////
    /// Unconditional jump
    j = 0x02,

    /// Jump and link
    jal = 0x03,
}

impl From<u8> for OpCode {
    fn from(v: u8) -> Self
    {
        use self::OpCode::*;

        match v
        {
            // addi
            0x08 => addi,

            // addiu
            0x09 => addiu,

            //andi
            0x0C => andi,

            // beq
            0x04 => beq,

            // bne
            0x05 => bne,

            // lbu
            0x24 => lbu,

            // lhu
            0x025 => lhu,

            // lui
            0x0F => lui,

            // lw
            0x23 => lw,

            // ori
            0x0D => ori,

            // sb
            0x28 => sb,

            // sh
            0x29 => sh,

            // slti
            0x0A => slti,

            // sw
            0x2B => sw,
            // j
            0x0 => j,

            // jal
            0x03 => jal,

            e => panic!("{} is not a valid opcode", e),
        }
    }
}

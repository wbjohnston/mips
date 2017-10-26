//! Mips Opcodes

/// A MIPS opcode
#[repr(u8)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    lhu = 0x25,

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
            0x02 => j,

            // jal
            0x03 => jal,

            e => panic!("{} is not a valid opcode", e),
        }
    }
}

#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn decode_addi()
    {
        let encoded = 0x08;
        assert_eq!(OpCode::addi, OpCode::from(encoded));
    }

    #[test]
    fn decode_addiu()
    {
        let encoded = 0x09;
        assert_eq!(OpCode::addiu, OpCode::from(encoded));
    }

    #[test]
    fn decode_andi()
    {
        let encoded = 0x0C;
        assert_eq!(OpCode::andi, OpCode::from(encoded));
    }

    #[test]
    fn decode_beq()
    {
        let encoded = 0x04;
        assert_eq!(OpCode::beq, OpCode::from(encoded));
    }

    #[test]
    fn decode_bne()
    {
        let encoded = 0x05;
        assert_eq!(OpCode::bne, OpCode::from(encoded));
    }

    #[test]
    fn decode_lbu()
    {
        let encoded = 0x24;
        assert_eq!(OpCode::lbu, OpCode::from(encoded));
    }

    #[test]
    fn decode_lhu()
    {
        let encoded = 0x25;
        assert_eq!(OpCode::lhu, OpCode::from(encoded));
    }

    #[test]
    fn decode_lui()
    {
        let encoded = 0x0F;
        assert_eq!(OpCode::lui, OpCode::from(encoded));
    }

    #[test]
    fn decode_lw()
    {
        let encoded = 0x23;
        assert_eq!(OpCode::lw, OpCode::from(encoded));
    }

    #[test]
    fn decode_ori()
    {
        let encoded = 0x0D;
        assert_eq!(OpCode::ori, OpCode::from(encoded));
    }

    #[test]
    fn decode_sb()
    {
        let encoded = 0x28;
        assert_eq!(OpCode::sb, OpCode::from(encoded));
    }

    #[test]
    fn decode_sh()
    {
        let encoded = 0x29;
        assert_eq!(OpCode::sh, OpCode::from(encoded));
    }

    #[test]
    fn decode_slti()
    {
        let encoded = 0x0A;
        assert_eq!(OpCode::slti, OpCode::from(encoded));
    }

    #[test]
    fn decode_sw()
    {
        let encoded = 0x2B;
        assert_eq!(OpCode::sw, OpCode::from(encoded));
    }

    #[test]
    fn decode_j()
    {
        let encoded = 0x02;
        assert_eq!(OpCode::j, OpCode::from(encoded));
    }

    #[test]
    fn decode_jal()
    {
        let encoded = 0x03;
        assert_eq!(OpCode::jal, OpCode::from(encoded));
    }
}

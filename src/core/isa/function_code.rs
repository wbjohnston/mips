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
            e => panic!("{} is not an opcode.", e),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn decode_add()
    {
        let encoded = 0x20;
        assert_eq!(FunctionCode::add, FunctionCode::from(encoded));
    }

    #[test]
    fn decode_addu()
    {
        let encoded = 0x21;
        assert_eq!(FunctionCode::addu, FunctionCode::from(encoded));
    }

    #[test]
    fn decode_and()
    {
        let encoded = 0x24;
        assert_eq!(FunctionCode::and, FunctionCode::from(encoded));
    }

    #[test]
    fn decode_div()
    {
        let encoded = 0x1A;
        assert_eq!(FunctionCode::div, FunctionCode::from(encoded));
    }

    #[test]
    fn decode_divu()
    {
        let encoded = 0x1B;
        assert_eq!(FunctionCode::divu, FunctionCode::from(encoded));
    }

    #[test]
    fn decode_mfhi()
    {
        let encoded = 0x10;
        assert_eq!(FunctionCode::mfhi, FunctionCode::from(encoded));
    }

    #[test]
    fn decode_mflo()
    {
        let encoded = 0x12;
        assert_eq!(FunctionCode::mflo, FunctionCode::from(encoded));
    }

    #[test]
    fn decode_nor()
    {
        let encoded = 0x27;
        assert_eq!(FunctionCode::nor, FunctionCode::from(encoded));
    }

    #[test]
    fn decode_xor()
    {
        let encoded = 0x26;
        assert_eq!(FunctionCode::xor, FunctionCode::from(encoded));
    }

    #[test]
    fn decode_or()
    {
        let encoded = 0x25;
        assert_eq!(FunctionCode::or, FunctionCode::from(encoded));
    }

    #[test]
    fn decode_slt()
    {
        let encoded = 0x2A;
        assert_eq!(FunctionCode::slt, FunctionCode::from(encoded));
    }

    #[test]
    fn decode_sltu()
    {
        let encoded = 0x2B;
        assert_eq!(FunctionCode::sltu, FunctionCode::from(encoded));
    }

    #[test]
    fn decode_sll()
    {
        let encoded = 0x00;
        assert_eq!(FunctionCode::sll, FunctionCode::from(encoded));
    }

    #[test]
    fn decode_sra()
    {
        let encoded = 0x02;
        assert_eq!(FunctionCode::srl, FunctionCode::from(encoded));
    }

    #[test]
    fn decode_sub()
    {
        let encoded = 0x22;
        assert_eq!(FunctionCode::sub, FunctionCode::from(encoded));
    }

    #[test]
    fn decode_subu()
    {
        let encoded = 0x23;
        assert_eq!(FunctionCode::subu, FunctionCode::from(encoded));
    }
}

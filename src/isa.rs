//! Instruction set architecture for MIPS assembly

// TODO(will): Are opcodes consistent across bit-width?
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

/// R type function code
#[repr(u8)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum RFunction {
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

// TODO(will): Need to implement 64 bit versions of each instruction

/// Jump type instruction, used for jumps
///
/// | OpCode | Target offset |
/// |--------|---------------|
/// | 0001xx | 26 bits       |
/// |--------|---------------|
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct JTypeInstruction {
    opcode: OpCode,
    offset: u32,
}

impl From<u32> for JTypeInstruction {
    #[allow(unused_variables)]
    fn from(v: u32) -> Self {
        unimplemented!("32-bit version not implemented yet");
    }
}

/// Register type instruction
///
/// | OpCode | Source 1 | Source 2 | Destination | Shift amount | Function |
/// |--------|----------|----------|-------------|--------------|----------|
/// | 000000 | 5 bits   | 5 bits   | 5 bits      | 5 bits       | 6 bits   |
/// |--------|----------|----------|-------------|--------------|----------|
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct RTypeInstruction {
    // opcode: u8 // Unused, guarenteed to be `00000`
    src1: u8,
    src2: u8,
    dst: u8,
    shift: u8,
    function: RFunction,
}

impl From<u32> for RTypeInstruction {
    #[allow(unused_variables)]
    fn from(v: u32) -> Self {
        unimplemented!("32-bit version not implemented yet");
    }
}

/// Immediate instruction (32-bit)
///
/// | OpCode | Source | Destination | Immediate Value |
/// |--------|--------|-------------|-----------------|
/// | 6 bits | 5 bits | 5 bits      | 16 bits         |
/// |--------|--------|-------------|-----------------|
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct ITypeInstruction32 {
    opcode: OpCode,
    src: u8,
    dst: u8,
    value: u16,
}

impl From<u32> for ITypeInstruction32 {
    #[allow(unused_variables)]
    fn from(v: u32) -> Self {
        unimplemented!("32-bit version not implemented yet");
    }
}

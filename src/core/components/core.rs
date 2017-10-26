//! MIPS processors

use super::{RegisterFile};

/// 32-bit MIPS `Core`
pub type Core32 = Core<u32>;

/// 64-bit MIPS `Core`
pub type Core64 = Core<u64>;

// TODO(will): pass in a type parameter for "endianess"
/// 32-bit MIPS core
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Core<WordType=u32> 
where
    WordType: Copy + Clone
{
    pc: WordType,
    ir: WordType,
    registers: RegisterFile<WordType>,
    float_registers: RegisterFile<WordType>,
    halted: bool,
    overflow_flag: bool
}

impl<WordType> Core<WordType>
where
    WordType: Copy + Clone
{
    /// Step the processor one cycle
    pub fn step(&mut self)
    {
        unimplemented!();
    }

    /// Get a copy of the program counter
    pub fn pc(&self) -> WordType
    {
        self.pc
    }

    /// Get the instruction Register
    pub fn ir(&self) -> WordType
    {
        self.ir
    }

    /// Get a copy of the Processor's `RegisterFile`
    pub fn register_file(&self) -> RegisterFile<WordType>
    {
        self.registers
    }

    /// Get a copy of the Processor's float `RegisterFile`
    pub fn float_register_file(&self) -> RegisterFile<WordType>
    {
        self.float_registers
    }

    /// Is the `Core` halted?
    pub fn is_halted(&self) -> bool
    {
        self.halted
    }

    /// Halt the `Core`
    pub fn halt(&mut self)
    {
        self.halted  = true;
    }

    /// Is the overflow flag set
    pub fn overflow(&self) -> bool
    {
        self.overflow_flag
    }
}

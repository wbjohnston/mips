//! MIPS processors

// NOTE(will): MIPS user memory is below 0x7fff_ffff
// NOTE(will): Program text starts at 0x0040_0000
// NOTE(will): data storage always starts at 0x1001_0000
// NOTE(will): MIPS execution cycle
//      IF : Instrucion fetch
//      ID : Decode
//      EX : Execute
//      MEM: Access memory and jump
//      WB : Write back

// use num_traits::{Unsigned, PrimInt};

use super::{RegisterFile};

use isa::Instruction32;

/// 32-bit MIPS `Core`
pub type Core32 = Core<u32>;

/// 64-bit MIPS `Core`
pub type Core64 = Core<u64>;

// TODO(will): pass in a type parameter for "endianess"
/// 32-bit MIPS core
#[derive(Clone, Debug, PartialEq)]
pub struct Core<WordType=u32> 
where
    WordType: Copy + Clone
{
    pc: WordType,

    ir: WordType,

    registers: RegisterFile<WordType>,

    float_registers: RegisterFile<WordType>,

    memory: Vec<WordType>,

    halted: bool,

    overflow_flag: bool
}

impl Core<u32>
{
    /// Step the processor one cycle
    pub fn step(&mut self)
    {
        //////////////////////////
        // Instruction fetch phase
        //////////////////////////
        self.ir = self.memory[self.pc as usize];
        self.pc += 4;

        //////////////////////////
        // Decode phase
        //////////////////////////
        let decoded = Instruction32::from(self.ir);

        //////////////////////////
        // Execute phase
        //////////////////////////
        // TODO(will): Write execute phase

        //////////////////////////
        // Memory access phase
        //////////////////////////
        // TODO(will): Memory access phase
        match decoded {
            Instruction32::R{..} => {}
            Instruction32::J{..} => {}
            Instruction32::I{..} => {}
            _ => {}
        }

        //////////////////////////
        // Writeback phase
        //////////////////////////
    }

    /// Get a copy of the program counter
    pub fn pc(&self) -> u32
    {
        self.pc
    }

    /// Get the instruction Register
    pub fn ir(&self) -> u32
    {
        self.ir
    }

    /// Get a copy of the Processor's `RegisterFile`
    pub fn register_file(&self) -> RegisterFile<u32>
    {
        self.registers
    }

    /// Get a copy of the Processor's float `RegisterFile`
    pub fn float_register_file(&self) -> RegisterFile<u32>
    {
        self.float_registers
    }

    /// Is the `Core` halted?
    pub fn is_halted(&self) -> bool
    {
        self.halted
    }

    /// Get a copy of the `Core`'s memoy
    pub fn memory(&self) -> Vec<u32>
    {
        self.memory.clone()
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

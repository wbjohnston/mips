//! MIPS processors

use std::mem;

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

use super::RegisterFile;
use isa::{Instruction32, FunctionCode, OpCode};

/// 32-bit MIPS `Core`
pub type Core32 = Core<u32>;

/// 64-bit MIPS `Core`
pub type Core64 = Core<u64>;

// TODO(will): pass in a type parameter for "endianess"
/// A MIPS core
#[derive(Clone, Debug, PartialEq)]
pub struct Core<WordType=u32> 
where
    WordType: Copy + Clone
{
    pc: WordType,

    ir: WordType,

    hi: WordType,

    lo: WordType,

    registers: RegisterFile<WordType>,

    float_registers: RegisterFile<WordType>,

    memory: Vec<WordType>,

    halted: bool,

    overflow_flag: bool,
}

impl Core<u32>
{
    /// Create a new core with a specified memory size
    pub fn with_memory_size(size: usize) -> Self
    {
        Self {
            pc: 0, // Where is initial pc
            ir: 0,
            hi: 0,
            lo: 0,
            registers: RegisterFile::new(),
            float_registers: RegisterFile::new(),
            memory: vec![0; size],
            halted: false,
            overflow_flag: false
        }
    }

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
        match decoded {
            Instruction32::R{src1, src2, dst, shift, func} => {
                use self::FunctionCode::*;
                let o1 = self.registers.fetch_by_idx(src1);
                let o2 = self.registers.fetch_by_idx(src2);

                match func {
                    add => self.registers.store_by_idx(
                        dst,
                        unsafe { mem::transmute_copy::<i32, u32>(
                            &(mem::transmute_copy::<u32, i32>(&o1) + 
                              mem::transmute_copy::<u32, i32>(&o2))
                        )}
                    ),
                    addu => self.registers.store_by_idx(dst, o1 + o2),
                    and => self.registers.store_by_idx(dst, o1 & o2),
                    div => self.registers.store_by_idx(
                        dst,
                        unsafe { mem::transmute_copy::<i32, u32>(
                            &(mem::transmute_copy::<u32, i32>(&o1) /
                              mem::transmute_copy::<u32, i32>(&o2))
                        )}
                    ),
                    divu => self.registers.store_by_idx(dst, o1 / o2),
                    mfhi => unimplemented!(),
                    mflo => unimplemented!(),
                    mult => unimplemented!(),
                    multu => self.registers.store_by_idx(dst, o1 * o2),
                    nor => self.registers.store_by_idx(dst, !(o1 | o2)),
                    or => self.registers.store_by_idx(dst, o1 | o2),
                    sll => self.registers.store_by_idx(dst, o1 << shift),
                    slt => unimplemented!(),
                    sltu => self.registers.store_by_idx(
                        dst,
                        if o1 < o2 { 1 } else { 0 }
                    ),
                    sra => unimplemented!(),
                    srl => unimplemented!(),
                    sub => unimplemented!(),
                    subu => self.registers.store_by_idx(dst, o1 - o2),
                    xor => self.registers.store_by_idx(dst, o1 ^ o2),
                }
            }
            Instruction32::J{opcode, offset} => {
                use self::OpCode::*;
                match opcode {
                    j => self.pc += offset,
                    jal => {
                        self.registers.store_by_idx(31, self.pc);
                        self.pc += offset;
                    },
                    _ => unreachable!()
                }
            }
            Instruction32::I{opcode, src, dst, imm} => {
                use self::OpCode::*;
                let operand = self.registers.fetch_by_idx(src);
                match opcode {
                    addi => unimplemented!(), 
                    addiu => self.registers.store_by_idx(
                        dst,
                        operand + imm as u32
                    ),
                    andi => self.registers.store_by_idx(
                        dst,
                        operand & imm as u32
                    ), 
                    beq => {
                        if src == 1 {
                            self.pc += imm as u32;
                        }
                    }, 
                    bne => unimplemented!(), 
                    ori => self.registers.store_by_idx(dst, operand | imm as u32), 
                    slti => self.registers.store_by_idx(
                        dst,
                        if operand < imm as u32 { 1 } else { 0 }
                    ), 
                    /* Memory access instructions */
                    lbu => { /* Fulfilled in MEM phase */ }, 
                    lhu => { /* Fulfilled in MEM phase */ }, 
                    lui => { /* Fulfilled in MEM phase */ }, 
                    lw => { /* Fulfilled in MEM phase */ }, 
                    sb => { /* Fulfilled in MEM phase */ }, 
                    sh => { /* Fulfilled in MEM phase*/ }, 
                    sw => { /* Fulfilled in MEM phase */ }, 
                    _ => unreachable!()
                }
            }
        }

        //////////////////////////
        // Memory access phase
        //////////////////////////
        // TODO(will): Memory access phase
        match decoded {
            Instruction32::R{..} => {}
            Instruction32::J{..} => {}
            Instruction32::I{..} => {}
        }

        //////////////////////////
        // Writeback phase
        //////////////////////////
        // TODO(will): implement writeback phase
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
    pub fn register(&self) -> RegisterFile<u32>
    {
        self.registers
    }

    /// Get a copy of the Processor's float `RegisterFile`
    pub fn float_registers(&self) -> RegisterFile<u32>
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

    /// Reset the `Core`
    pub fn reset(&mut self)
    {
        unimplemented!();
    }

    /// Is the overflow flag set
    pub fn overflow(&self) -> bool
    {
        self.overflow_flag
    }
}

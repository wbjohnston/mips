//! Mips processors

use super::{RegisterFile32, RegisterFile64};
use super::{FloatRegisterFile32, FloatRegisterFile64};

/// 32-bit MIPS core
#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub struct Core32 {
    pc: u32,
    ir: u32,
    registers: RegisterFile32,
    float_registers: FloatRegisterFile32,
}

/// 32-bit MIPS core
#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub struct Core64 {
    pc: u64,
    ir: u64,
    registers: RegisterFile64,
    float_registers: FloatRegisterFile64,
}

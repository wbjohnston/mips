//! Instruction set archicecture for mips assembly

// TODO(will): Need to implement 64 bit versions of each instruction

/// J-type instruction, used for jumps
pub struct JTypeInstruction
{
    // TODO(will): fields
}

impl From<u32> for JTypeInstruction
{
    #[allow(unused_variables)]
    fn from(v: u32) -> Self
    {
        unimplemented!("32-bit version not implemented yet");
    }
}

impl From<u64> for JTypeInstruction
{
    #[allow(unused_variables)]
    fn from(v: u64) -> Self
    {
        unimplemented!("64-bit version not implemented yet");
    }
}

/// R-type instruction, used for multi-register instructions
pub struct RTypeInstruction
{
    // TODO(will): fields
}

impl From<u32> for RTypeInstruction
{
    #[allow(unused_variables)]
    fn from(v: u32) -> Self
    {
        unimplemented!("32-bit version not implemented yet");
    }
}

impl From<u64> for RTypeInstruction
{
    #[allow(unused_variables)]
    fn from(v: u64) -> Self
    {
        unimplemented!("64-bit version not implemented yet");
    }
}

/// I-type instruction, used for instructions with immediate values
pub struct ITypeInstruction
{
    // TODO(will): fields
}

impl From<u32> for ITypeInstruction
{
    #[allow(unused_variables)]
    fn from(v: u32) -> Self
    {
        unimplemented!("32-bit version not implemented yet");
    }
}

impl From<u64> for ITypeInstruction
{
    #[allow(unused_variables)]
    fn from(v: u64) -> Self
    {
        unimplemented!("64-bit version not implemented yet");
    }
}

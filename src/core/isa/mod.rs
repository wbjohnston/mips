//! Instruction set types

mod opcode;
pub use self::opcode::OpCode;

mod instruction;
pub use self::instruction::Instruction32;

mod function_code;
pub use self::function_code::FunctionCode;

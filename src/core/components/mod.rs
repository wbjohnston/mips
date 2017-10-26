//! Components of a MIPS systems

mod core;
pub use self::core::Core32;

mod register_file;
pub use self::register_file::RegisterFile32;
pub use self::register_file::RegisterFile64;
pub use self::register_file::FloatRegisterFile32;
pub use self::register_file::FloatRegisterFile64;

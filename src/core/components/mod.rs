//! Components of a MIPS systems

mod core;
pub use self::core::{Core, Core32, Core64};

mod register_file;
pub use self::register_file::RegisterFile;

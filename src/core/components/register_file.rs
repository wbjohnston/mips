//! Register files for 32 and 64 bit MIPS architecture

/// A 32-bit register file
#[allow(dead_code)]
pub type RegisterFile32 = RegisterFile<u32>;

/// A 64-bit register file
#[allow(dead_code)]
pub type RegisterFile64 = RegisterFile<u64>;

/// A Mips register file
#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub struct RegisterFile<T = u32> {
    zero: T,
    at: T,

    // Return registers
    v0: T,
    v1: T,

    // Argument registers
    a0: T,
    a1: T,
    a2: T,
    a3: T,

    // Temp data registers
    t0: T,
    t1: T,
    t2: T,
    t3: T,
    t4: T,
    t5: T,
    t6: T,
    t7: T,

    // Saved data registers
    s0: T,
    s1: T,
    s2: T,
    s3: T,
    s4: T,
    s5: T,
    s6: T,
    s7: T,

    // Temp data registers cont.
    t8: T,
    t9: T,

    // Kernel registers
    k0: T,
    k1: T,

    // Global area pointer
    gp: T,

    // Stack Pointer register
    sp: T,

    // Frame pointer register
    fp: T,

    // Return address register
    ra: T,
}

/// A 32-bit floating point register file
#[allow(dead_code)]
pub type FloatRegisterFile32 = FloatRegisterFile<f32>;

/// A 64-bit floating point register file
#[allow(dead_code)]
pub type FloatRegisterFile64 = FloatRegisterFile<f64>;

/// Floating point register file
#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub struct FloatRegisterFile<T = f32> {
    // Return value registers
    f0: T,
    f2: T,
    f3: T,

    // Temporary data registers
    f4: T,
    f5: T,
    f6: T,
    f7: T,
    f8: T,
    f9: T,
    f10: T,

    f11: T, // ???

    // Argument registers
    f12: T,
    f13: T,
    f14: T,

    f15: T, // ???

    // Temeporary data registers cont.
    f16: T,
    f17: T,
    f18: T,

    f19: T, // ???

    // Saved registers
    f20: T,
    f21: T,
    f22: T,
    f23: T,
    f24: T,
    f25: T,
    f26: T,
    f27: T,
    f28: T,
    f29: T,
    f30: T,
    f31: T,
}

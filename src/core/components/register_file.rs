//! Register files for 32 and 64 bit MIPS architecture

/// A MIPS register file
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

//! Register files for 32 and 64 bit MIPS architecture

/// A MIPS register file
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RegisterFile<WordType = u32> 
where 
    WordType: Copy + Clone
{
    zero: WordType,
    at: WordType,

    // Return registers
    v0: WordType,
    v1: WordType,

    // Argument registers
    a0: WordType,
    a1: WordType,
    a2: WordType,
    a3: WordType,

    // WordTypeemp data registers
    t0: WordType,
    t1: WordType,
    t2: WordType,
    t3: WordType,
    t4: WordType,
    t5: WordType,
    t6: WordType,
    t7: WordType,

    // Saved data registers
    s0: WordType,
    s1: WordType,
    s2: WordType,
    s3: WordType,
    s4: WordType,
    s5: WordType,
    s6: WordType,
    s7: WordType,

    // WordTypeemp data registers cont.
    t8: WordType,
    t9: WordType,

    // Kernel registers
    k0: WordType,
    k1: WordType,

    // Global area pointer
    gp: WordType,

    // Stack Pointer register
    sp: WordType,

    // Frame pointer register
    fp: WordType,

    // Return address register
    ra: WordType,
}

impl<WordType> RegisterFile<WordType>
where
    WordType: Copy + Clone
{
    /// Fetch a register by a register index [0..31]
    pub fn fetch_by_idx(&self, idx: u8) -> WordType
    {
        match idx {
            0 => self.zero,
            1 => self.at,
            2 => self.v0,
            3 => self.v1,
            4 => self.a0,
            5 => self.a1,
            6 => self.a2,
            7 => self.a3,
            8 => self.t0,
            9 => self.t1,
            10 => self.t2,
            11 => self.t3,
            12 => self.t4,
            13 => self.t5,
            14 => self.t6,
            15 => self.t7,
            16 => self.s0,
            17 => self.s1,
            18 => self.s2,
            19 => self.s3,
            20 => self.s4,
            21 => self.s5,
            22 => self.s6,
            23 => self.s7,
            24 => self.t8,
            25 => self.t9,
            26 => self.k0,
            27 => self.k1,
            28 => self.gp,
            29 => self.sp,
            30 => self.fp,
            31 => self.ra,
            idx => panic!("{} is not a valid register address", idx)
        }
    }

    /// Store a value using an index [0..31]
    pub fn store_by_idx(&mut self, idx: u8, val: WordType)
    {
        // TODO(will): this style of address translation is super dumb
        match idx {
            0 => self.zero = val,
            1 => self.at = val,
            2 => self.v0 = val,
            3 => self.v1 = val,
            4 => self.a0 = val,
            5 => self.a1 = val,
            6 => self.a2 = val,
            7 => self.a3 = val,
            8 => self.t0 = val,
            9 => self.t1 = val,
            10 => self.t2 = val,
            11 => self.t3 = val,
            12 => self.t4 = val,
            13 => self.t5 = val,
            14 => self.t6 = val,
            15 => self.t7 = val,
            16 => self.s0 = val,
            17 => self.s1 = val,
            18 => self.s2 = val,
            19 => self.s3 = val,
            20 => self.s4 = val,
            21 => self.s5 = val,
            22 => self.s6 = val,
            23 => self.s7 = val,
            24 => self.t8 = val,
            25 => self.t9 = val,
            26 => self.k0 = val,
            27 => self.k1 = val,
            28 => self.gp = val,
            29 => self.sp = val,
            30 => self.fp = val,
            31 => self.ra = val,
            idx => panic!("{} is not a valid register address", idx)
        };
    }
}

impl RegisterFile<u32>
{
    /// Create a new `RegisterFile`
    pub fn new() -> Self
    {
        Self {
            zero: 0,
            at: 0,
            v0: 0,
            v1: 0,
            a0: 0,
            a1: 0,
            a2: 0,
            a3: 0,
            t0: 0,
            t1: 0,
            t2: 0,
            t3: 0,
            t4: 0,
            t5: 0,
            t6: 0,
            t7: 0,
            s0: 0,
            s1: 0,
            s2: 0,
            s3: 0,
            s4: 0,
            s5: 0,
            s6: 0,
            s7: 0,
            t8: 0,
            t9: 0,
            k0: 0,
            k1: 0,
            gp: 0,
            sp: 0,
            fp: 0,
            ra: 0
        }
    }
}

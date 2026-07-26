/// Based on the 8^8 space 8 opcodes per vm, which a ISA of 8 different opcodes.
pub const MAX_SEARCH_SPACE: u32 = 16_777_216;

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum IgnoreMask {
    Ignored = 0,
    Checked = u32::MAX,
}

/// Structure of Arrays (SoA) layout aligned to 64-byte cache lines
/// to ensure contiguous, prefetcher-friendly linear access patterns.
#[repr(align(64))]
pub struct TrainingDataset<const TEST_SETS: usize> {
    pub reg0_inputs: [u32; TEST_SETS],
    pub reg1_inputs: [u32; TEST_SETS],
    pub reg2_inputs: [u32; TEST_SETS],
    pub reg3_inputs: [u32; TEST_SETS],

    pub reg0_expected: [u32; TEST_SETS],
    pub reg1_expected: [u32; TEST_SETS],
    pub reg2_expected: [u32; TEST_SETS],
    pub reg3_expected: [u32; TEST_SETS],

    pub reg0_ignore: [IgnoreMask; TEST_SETS],
    pub reg1_ignore: [IgnoreMask; TEST_SETS],
    pub reg2_ignore: [IgnoreMask; TEST_SETS],
    pub reg3_ignore: [IgnoreMask; TEST_SETS],
}

impl<const TEST_SETS: usize> Default for TrainingDataset<TEST_SETS> {
    #[inline(always)]
    fn default() -> Self {
        Self {
            reg0_inputs: [0; TEST_SETS],
            reg1_inputs: [0; TEST_SETS],
            reg2_inputs: [0; TEST_SETS],
            reg3_inputs: [0; TEST_SETS],

            reg0_expected: [0; TEST_SETS],
            reg1_expected: [0; TEST_SETS],
            reg2_expected: [0; TEST_SETS],
            reg3_expected: [0; TEST_SETS],

            reg0_ignore: [IgnoreMask::Checked; TEST_SETS],
            reg1_ignore: [IgnoreMask::Checked; TEST_SETS],
            reg2_ignore: [IgnoreMask::Checked; TEST_SETS],
            reg3_ignore: [IgnoreMask::Checked; TEST_SETS],
        }
    }
}

impl<const TEST_SETS: usize> TrainingDataset<TEST_SETS> {
    pub fn to_bytes(self) -> Vec<u32> {
        let mut r: Vec<u32> = Vec::with_capacity((TEST_SETS * 12) + 1);

        r.push(TEST_SETS as u32);
        r.extend_from_slice(&self.reg0_inputs);
        r.extend_from_slice(&self.reg1_inputs);
        r.extend_from_slice(&self.reg2_inputs);
        r.extend_from_slice(&self.reg3_inputs);
        r.extend_from_slice(&self.reg0_expected);
        r.extend_from_slice(&self.reg1_expected);
        r.extend_from_slice(&self.reg2_expected);
        r.extend_from_slice(&self.reg3_expected);
        r.extend_from_slice(unsafe {
            std::slice::from_raw_parts(self.reg0_ignore.as_ptr() as *const u32, TEST_SETS)
        });
        r.extend_from_slice(unsafe {
            std::slice::from_raw_parts(self.reg1_ignore.as_ptr() as *const u32, TEST_SETS)
        });
        r.extend_from_slice(unsafe {
            std::slice::from_raw_parts(self.reg2_ignore.as_ptr() as *const u32, TEST_SETS)
        });
        r.extend_from_slice(unsafe {
            std::slice::from_raw_parts(self.reg3_ignore.as_ptr() as *const u32, TEST_SETS)
        });

        r
    }

    pub fn new() -> Self {
        Self::default()
    }
}

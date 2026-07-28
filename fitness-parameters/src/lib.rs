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
pub struct Requirements<const TEST_SETS: usize> {
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

impl<const TEST_SETS: usize> Default for Requirements<TEST_SETS> {
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

impl<const TEST_SETS: usize> Requirements<TEST_SETS> {
    pub fn to_bytes_form(&self) -> Vec<u8> {
        let num_u32s = (TEST_SETS * 12) + 1;
        let byte_capacity = num_u32s * std::mem::size_of::<u32>();

        let mut r_bytes: Vec<u8> = Vec::with_capacity(byte_capacity);

        r_bytes.extend_from_slice(&(TEST_SETS as u32).to_ne_bytes());

        let extend_u32_slice = |vec: &mut Vec<u8>, slice: &[u32]| {
            let raw_bytes = unsafe {
                std::slice::from_raw_parts(
                    slice.as_ptr() as *const u8,
                    slice.len() * std::mem::size_of::<u32>(),
                )
            };
            vec.extend_from_slice(raw_bytes);
        };

        extend_u32_slice(&mut r_bytes, &self.reg0_inputs);
        extend_u32_slice(&mut r_bytes, &self.reg1_inputs);
        extend_u32_slice(&mut r_bytes, &self.reg2_inputs);
        extend_u32_slice(&mut r_bytes, &self.reg3_inputs);
        extend_u32_slice(&mut r_bytes, &self.reg0_expected);
        extend_u32_slice(&mut r_bytes, &self.reg1_expected);
        extend_u32_slice(&mut r_bytes, &self.reg2_expected);
        extend_u32_slice(&mut r_bytes, &self.reg3_expected);

        extend_u32_slice(&mut r_bytes, unsafe {
            std::slice::from_raw_parts(self.reg0_ignore.as_ptr() as *const u32, TEST_SETS)
        });
        extend_u32_slice(&mut r_bytes, unsafe {
            std::slice::from_raw_parts(self.reg1_ignore.as_ptr() as *const u32, TEST_SETS)
        });
        extend_u32_slice(&mut r_bytes, unsafe {
            std::slice::from_raw_parts(self.reg2_ignore.as_ptr() as *const u32, TEST_SETS)
        });
        extend_u32_slice(&mut r_bytes, unsafe {
            std::slice::from_raw_parts(self.reg3_ignore.as_ptr() as *const u32, TEST_SETS)
        });

        r_bytes
    }

    pub fn new() -> Self {
        Self::default()
    }
}

/// Based on the 8^8 space 8 opcodes per vm, which a ISA of 8 different opcodes.
pub const MAX_SEARCH_SPACE: u32 = 16_777_216;

#[repr(u32)]
pub enum IgnoreMask {
    Ignored = u32::MAX,
    Checked = 0,
}

/// Simple DOD holder for tests on mutants.
pub struct TrainingDataset<const TEST_SETS: usize> {
    pub register_inputs: [[u32; 4]; TEST_SETS],
    pub expected_outputs: [[u32; 4]; TEST_SETS],
    pub ignore_outputs: [[IgnoreMask; 4]; TEST_SETS],
}

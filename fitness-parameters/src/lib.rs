pub const MAX_SEARCH_SPACE: u32 = 16777216;

pub struct TestDataset<const TEST_SETS: usize> {
    pub register_inputs: [[u32; 4]; TEST_SETS],
    pub expected_outputs: [[u32; 4]; TEST_SETS],
}

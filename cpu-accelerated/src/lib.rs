use drakon_opcodes::*;

/// Cache lane 64 byte aligned layout:
#[repr(C, align(64))]
pub struct OpcodeLane {
    pub vm_genes: [[u8; 8]; 8], // 8 VMs each with 8-8bit word max instructions.
}

// This ticks entire OpcodeLane at once, with optimized ALU port usage.
#[inline(always)]
pub fn tick_lane(lane: &OpcodeLane, register_preload_input: &[u32; 4]) -> [[u32; 4]; 4] {
    // 16 Registers used
    let mut vm_regs_bank: [[u32; 4]; 4] = [register_preload_input.clone(); 4];

    for i in 0..8 {
        for j in 0..8 {
            let instruction = lane.vm_genes[i][j];
        }
    }

    vm_regs_bank
}

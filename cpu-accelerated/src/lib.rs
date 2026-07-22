use drakon_opcodes::*;

const MAX_INSTRUCTIONS: usize = 8;

/// Cache lane 64 byte aligned layout:
/// VM_0[Inst*4Bytes: u32, Inst*4Bytes: u32]
/// VM_1[Inst*4Bytes: u32, Inst*4Bytes: u32]
/// VM_2[Inst*4Bytes: u32, Inst*4Bytes: u32]
/// VM_3[Inst*4Bytes: u32, Inst*4Bytes: u32]
#[repr(C, align(64))]
pub struct OpcodeLane {
    pub genomes: [u32; 16],
}

// This ticks entire OpcodeLane at once, with optimized ALU port usage.
#[inline(always)]
pub fn tick_lane(lane: OpcodeLane) -> [[u32; 4]; 4] {
    let mut vm_regs_0: [u32; 4] = [0, 0, 0, 0];
    let mut vm_regs_1: [u32; 4] = [0, 0, 0, 0];
    let mut vm_regs_2: [u32; 4] = [0, 0, 0, 0];
    let mut vm_regs_3: [u32; 4] = [0, 0, 0, 0];

    for i in 0..MAX_INSTRUCTIONS {
        // Should be unrolled by compiler!
        for j in 0..3 {
            lane.vm_0
        }
    }

    [vm_regs_0, vm_regs_1, vm_regs_2, vm_regs_3] // Cache Aligned as well!
}

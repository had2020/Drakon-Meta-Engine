use std::mem::transmute;

use drakon_opcodes::*;

type OpHandler = fn(u32, u32) -> u32;

static DISPATCH_TABLE: [OpHandler; 16] = [
    |a, b| if a != 0 { b } else { a }, // CMOV
    |a, b| a.min(b),                   // MIN
    |a, b| a + b,                      // IMM
    |_, b| b,                          // MOV
    |a, b| a & b,                      // AND
    |a, b| a | b,                      // OR
    |a, _| !a,                         // NOT
    |a, b| a ^ b,                      // XOR
    |a, b| a >> b,                     // SHR
    |a, b| a << b,                     // SHL
    |a, b| a.rotate_right(b),          // ROR
    |a, b| a.rotate_left(b),           // ROL
    |a, b| a.wrapping_add(b),          // ADD
    |a, b| a.wrapping_sub(b),          // SUB
    |a, b| a.max(b),                   // MAX
    |_, b| b.count_ones(),             // POPCNT
];

/// Cache lane 64 byte aligned layout:
#[repr(C, align(64))]
pub struct OpcodeLane {
    pub vm_genes: [[u8; 8]; 4], // 4 VMs each with 8-8bit word max instructions.
}

// This ticks entire OpcodeLane at once, with optimized ALU port usage.
#[inline(always)]
pub fn tick_lane(lane: &OpcodeLane, register_preload_input: &[u32; 4]) -> [[u32; 4]; 4] {
    // 16 Registers used
    let mut vm_regs_bank: [[u32; 4]; 4] = [register_preload_input.clone(); 4];

    // VM level
    for i in 0..4 {
        // Opcode level
        for j in 0..8 {
            let instruction = lane.vm_genes[i][j];
            let opcode: DrakonOpBase = unsafe { transmute(instruction >> 4) };
            let dist_reg = (instruction & 0xF0) >> 2;
            let reg = instruction & 0xFC;

            unsafe {
                vm_regs_bank[i][dist_reg as usize] =
                    DISPATCH_TABLE.get_unchecked(opcode as usize)(dist_reg as u32, reg as u32);
            }
        }
    }

    vm_regs_bank
}

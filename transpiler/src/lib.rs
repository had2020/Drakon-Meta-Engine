use drakon_opcodes::*;
use std::fmt::Write;

#[repr(C, align(64))]
#[derive(Debug, Clone, Copy)]
pub struct FittestProgram {
    pub instructions: [u8; 8],
}

impl FittestProgram {
    pub fn out_rustc(&self) -> String {
        // Pre-allocate buffer on heap to avoid allocations during formatting
        let mut code = String::with_capacity(1024);

        code.push_str("pub fn generated_program(regs: &mut [u32; 4]) {\n");

        for (idx, &byte) in self.instructions.iter().enumerate() {
            // Field extraction via bitmasking
            // Memory layout: [4bits OPCODE][2bits DEST_REG][2bits SRC_REG]
            let op_code: DrakonOpBase = Instruction::extract_opcode(byte);
            let dst = (byte >> 2) & 0x03;
            let src = byte & 0x03;

            let _ = write!(code, "    // Instr {idx}: {op_code:?} r{dst}, r{src}\n    ");

            match op_code {
                // Conditional Move: Dest = (Src != 0) ? Dest : Src (or custom logic)
                DrakonOpBase::CMOV => {
                    let _ = writeln!(code, "if regs[{dst}] != 0 {{ regs[{dst}] = regs[{src}]; }}");
                }
                DrakonOpBase::MIN => {
                    let _ = writeln!(code, "regs[{dst}] = regs[{dst}].min(regs[{src}]);");
                }
                DrakonOpBase::IMM => {
                    // Treating `src` bitfield as a raw 2-bit immediate (0..=3)
                    let _ = writeln!(code, "regs[{dst}] += {src}_u32;");
                }
                DrakonOpBase::MOV => {
                    let _ = writeln!(code, "regs[{dst}] = regs[{src}];");
                }
                DrakonOpBase::AND => {
                    let _ = writeln!(code, "regs[{dst}] &= regs[{src}];");
                }
                DrakonOpBase::OR => {
                    let _ = writeln!(code, "regs[{dst}] |= regs[{src}];");
                }
                DrakonOpBase::NOT => {
                    let _ = writeln!(code, "regs[{dst}] = !regs[{dst}];");
                }
                DrakonOpBase::XOR => {
                    let _ = writeln!(code, "regs[{dst}] ^= regs[{src}];");
                }
                DrakonOpBase::SHR => {
                    let _ = writeln!(code, "regs[{dst}] = regs[{dst}].wrapping_shr(regs[{src}]);");
                }
                DrakonOpBase::SHL => {
                    let _ = writeln!(code, "regs[{dst}] = regs[{dst}].wrapping_shl(regs[{src}]);");
                }
                DrakonOpBase::ROR => {
                    let _ = writeln!(code, "regs[{dst}] = regs[{dst}].rotate_right(regs[{src}]);");
                }
                DrakonOpBase::ROL => {
                    let _ = writeln!(code, "regs[{dst}] = regs[{dst}].rotate_left(regs[{src}]);");
                }
                DrakonOpBase::ADD => {
                    let _ = writeln!(code, "regs[{dst}] = regs[{dst}].wrapping_add(regs[{src}]);");
                }
                DrakonOpBase::SUB => {
                    let _ = writeln!(code, "regs[{dst}] = regs[{dst}].wrapping_sub(regs[{src}]);");
                }
                DrakonOpBase::MAX => {
                    let _ = writeln!(code, "regs[{dst}] = regs[{dst}].max(regs[{src}]);");
                }
                DrakonOpBase::POPCNT => {
                    let _ = writeln!(code, "regs[{dst}] = regs[{src}].count_ones();");
                }
            }
        }

        code.push_str("}\n");
        code
    }
}

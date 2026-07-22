// [4bits OPCODE][2bits DIST_REG][2bits REG]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrakonOpBase {
    CMOV = 0, // Conditional Move (Dest = Cond ? Src : Dest)
    MIN = 1,
    IMM = 2,
    MOV = 3,
    AND = 4,
    OR = 5,
    NOT = 6,
    XOR = 7,
    SHR = 8,
    SHL = 9,
    ROR = 10,
    ROL = 11,
    ADD = 12,
    SUB = 13,
    MAX = 14,
    POPCNT = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Instruction {
    pub op: DrakonOpBase,
    pub dist_reg: u8,
    pub reg: u8,
}

impl Instruction {
    const REG_MASK: u8 = 0b0000_0011;
    const DIST_REG_MASK: u8 = 0b0000_1100;

    pub const fn pack(&self) -> u8 {
        ((self.op as u8) << 4)
            | ((self.dist_reg & Self::DIST_REG_MASK) << 2)
            | (self.reg & Self::REG_MASK)
    }

    pub const fn unpack(packed: u8) -> Self {
        let op = unsafe { std::mem::transmute(packed >> 4) };
        Self {
            op,
            dist_reg: (packed >> 2) & Self::DIST_REG_MASK,
            reg: packed & Self::REG_MASK,
        }
    }

    /// Pack 4 instructions into a u32 (Little-Endian layout)
    pub fn pack_u32(insts: [Instruction; 4]) -> u32 {
        (insts[0].pack() as u32)
            | ((insts[1].pack() as u32) << 8)
            | ((insts[2].pack() as u32) << 16)
            | ((insts[3].pack() as u32) << 24)
    }

    /// Unpack a u32 into 4 instructions (SWAR parsing)
    pub fn unpack_u32(packed: u32) -> [Instruction; 4] {
        let bytes = packed.to_le_bytes();
        [
            Instruction::unpack(bytes[0]),
            Instruction::unpack(bytes[1]),
            Instruction::unpack(bytes[2]),
            Instruction::unpack(bytes[3]),
        ]
    }

    /// Pack 8 instructions into a u64
    pub fn pack_u64(insts: [Instruction; 8]) -> u64 {
        (insts[0].pack() as u64)
            | ((insts[1].pack() as u64) << 8)
            | ((insts[2].pack() as u64) << 16)
            | ((insts[3].pack() as u64) << 24)
            | ((insts[4].pack() as u64) << 32)
            | ((insts[5].pack() as u64) << 40)
            | ((insts[6].pack() as u64) << 48)
            | ((insts[7].pack() as u64) << 56)
    }

    /// Unpack a u64 into 8 instructions
    pub fn unpack_u64(packed: u64) -> [Instruction; 8] {
        let bytes = packed.to_le_bytes();
        [
            Instruction::unpack(bytes[0]),
            Instruction::unpack(bytes[1]),
            Instruction::unpack(bytes[2]),
            Instruction::unpack(bytes[3]),
            Instruction::unpack(bytes[4]),
            Instruction::unpack(bytes[5]),
            Instruction::unpack(bytes[6]),
            Instruction::unpack(bytes[7]),
        ]
    }
}

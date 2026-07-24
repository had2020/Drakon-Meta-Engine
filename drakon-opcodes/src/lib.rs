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
    pub fn extract_opcode(byte: u8) -> DrakonOpBase {
        unsafe { std::mem::transmute(byte >> 4) }
    }

    const REG_MASK: u8 = 0b0000_0011;
    const DIST_REG_MASK: u8 = 0b0000_1100;

    pub const fn pack(&self) -> u8 {
        ((self.op as u8) << 4)
            | ((self.dist_reg & Self::DIST_REG_MASK) << 2)
            | (self.reg & Self::REG_MASK)
    }

    pub const fn unpack_to_readable(packed: u8) -> Self {
        let op = unsafe { std::mem::transmute(packed >> 4) };
        Self {
            op,
            dist_reg: (packed >> 2) & Self::DIST_REG_MASK,
            reg: packed & Self::REG_MASK,
        }
    }

    /// Pack 4 instructions into a u32 (Little-Endian layout)
    pub fn pack_readable_u32(insts: [Instruction; 4]) -> u32 {
        (insts[0].pack() as u32)
            | ((insts[1].pack() as u32) << 8)
            | ((insts[2].pack() as u32) << 16)
            | ((insts[3].pack() as u32) << 24)
    }

    pub fn pack_bytes_u32(insts: [u8; 4]) -> u32 {
        u32::from_ne_bytes(insts)
    }

    /// Unpack a u32 into 4 instructions (SWAR parsing)
    pub fn unpack_u32_to_bytes(packed: u32) -> [Instruction; 4] {
        let bytes = packed.to_le_bytes();
        [
            Instruction::unpack_to_readable(bytes[0]),
            Instruction::unpack_to_readable(bytes[1]),
            Instruction::unpack_to_readable(bytes[2]),
            Instruction::unpack_to_readable(bytes[3]),
        ]
    }
}

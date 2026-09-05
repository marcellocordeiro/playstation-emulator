use std::fmt::Display;

use crate::components::cpu::decoded_instruction::DecodedInstruction;

#[derive(Clone, Copy)]
pub struct Instruction(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegisterIndex(pub u32);

impl Display for RegisterIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self.0 {
            0 => "zero",
            1 => "at",
            2 => "v0",
            3 => "v1",
            4 => "a0",
            5 => "a1",
            6 => "a2",
            7 => "a3",
            8 => "t0",
            9 => "t1",
            10 => "t2",
            11 => "t3",
            12 => "t4",
            13 => "t5",
            14 => "t6",
            15 => "t7",
            16 => "s0",
            17 => "s1",
            18 => "s2",
            19 => "s3",
            20 => "s4",
            21 => "s5",
            22 => "s6",
            23 => "s7",
            24 => "t8",
            25 => "t9",
            26 => "k0",
            27 => "k1",
            28 => "gp",
            29 => "sp",
            30 => "fp",
            31 => "ra",

            invalid => {
                return write!(f, "r{invalid}");
            }
        };

        f.write_str(str)
    }
}

impl Instruction {
    /// Primary opcode field, or operation code (op)
    ///
    /// Bits 31...26 (6 bits)
    #[must_use]
    pub fn primary(self) -> u32 {
        self.0 >> 26
    }

    /// Secondary opcode field (when primary opcode == 00h), or function field (funct)
    ///
    /// Bits 5...0 (6 bits)
    #[must_use]
    pub fn secondary(self) -> u32 {
        self.0 & 0x3F
    }

    #[must_use]
    pub fn cop_opcode(self) -> u32 {
        self.rs().0
    }

    /// Immediate value
    ///
    /// Bits 15...0 (16 bits)
    #[must_use]
    pub fn imm(self) -> u32 {
        self.0 & 0xFFFF
    }

    /// Sign extended immediate value
    ///
    /// Bits 15...0 (16 bits)
    #[must_use]
    pub fn imm_sign_extended(self) -> u32 {
        (self.0 & 0xFFFF) as i16 as u32
    }

    /// Jump target, already shifted to be u32 aligned
    #[must_use]
    pub fn jump_target(self) -> u32 {
        // 1:0 Assumed to be 0 due to 32 bits alignment
        (self.0 & 0x03FF_FFFF) << 2
    }

    #[must_use]
    pub fn branch_offset(self) -> u32 {
        self.imm_sign_extended() << 2
    }

    /// Shift immediate value (imm5, or shamt)
    ///
    /// Bits 10...6 (5 bits)
    #[must_use]
    pub fn shift_imm(self) -> u32 {
        (self.0 >> 6) & 0x1F
    }

    /// Register index rs (source register (operand))
    ///
    /// Bits 25...21 (5 bits)
    #[must_use]
    pub fn rs(self) -> RegisterIndex {
        RegisterIndex((self.0 >> 21) & 0x1F)
    }

    /// Register indexrst (second source register)
    ///
    /// Bits 20...16 (5 bits)
    #[must_use]
    pub fn rt(self) -> RegisterIndex {
        RegisterIndex((self.0 >> 16) & 0x1F)
    }

    /// Register index rd (destination register)
    ///
    /// Bits 15...11 (5 bits)
    #[must_use]
    pub fn rd(self) -> RegisterIndex {
        RegisterIndex((self.0 >> 11) & 0x1F)
    }

    #[must_use]
    pub fn decoded(self) -> DecodedInstruction {
        DecodedInstruction::decode(self)
    }
}

impl std::fmt::Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Instruction {:#010X}, primary opcode = {:#04X}, secondary opcode = {:#04X}",
            self.0,
            self.primary(),
            self.secondary()
        ))
    }
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

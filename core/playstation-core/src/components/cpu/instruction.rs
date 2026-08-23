#[derive(Clone, Copy)]
pub struct Instruction(pub u32);
pub struct RegisterIndex(pub u32);

impl Instruction {
    /// Primary opcode field
    ///
    /// Bits 31...26 (6 bits)
    pub fn primary(self) -> u32 {
        self.0 >> 26
    }

    /// Secondary opcode field (when Primary opcode == 00h)
    ///
    /// Bits 5...0 (6 bits)
    pub fn secondary(self) -> u32 {
        self.0 & 0x3F
    }

    /// Immediate value
    ///
    /// Bits 15...0 (16 bits)
    pub fn imm(self) -> u32 {
        self.0 & 0xFFFF
    }

    /// Sign extended immediate value
    ///
    /// Bits 15...0 (16 bits)
    pub fn imm_sign_extended(self) -> u32 {
        let value = (self.0 & 0xFFFF) as i16;

        value as u32
    }

    pub fn imm_jump(self) -> u32 {
        self.0 & 0x03FF_FFFF
    }

    /// Shift immediate value (imm5)
    ///
    /// Bits 10...6 (5 bits)
    pub fn shift_imm(self) -> u32 {
        (self.0 >> 6) & 0x1F
    }

    /// Register index s
    ///
    /// Bits 25...11 (5 bits)
    pub fn rs(self) -> RegisterIndex {
        RegisterIndex((self.0 >> 21) & 0x1F)
    }

    /// Register index t
    ///
    /// Bits 20...16 (5 bits)
    pub fn rt(self) -> RegisterIndex {
        RegisterIndex((self.0 >> 16) & 0x1F)
    }

    /// Register index d
    ///
    /// Bits 15...11 (5 bits)
    pub fn rd(self) -> RegisterIndex {
        RegisterIndex((self.0 >> 1) & 0x1F)
    }
}

impl std::fmt::Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Instruction {:#010x}, primary opcode = {:#04x}, secondary opcode = {:#04x}",
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

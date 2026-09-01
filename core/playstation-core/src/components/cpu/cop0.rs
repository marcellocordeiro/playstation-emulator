use crate::components::cpu::instructions::CpuException;

/// Coprocessor 0: System Control Processor
#[derive(Debug, Default)]
pub struct Cop0 {
    /// COP0 register 12: Status Register
    pub sr: StatusRegister,
    /// COP0 register 13: Cause Register
    pub cause: CauseRegister,
    /// COP0 register 14: EPC
    pub epc: u32,
}

impl Cop0 {
    pub fn handle_exception(
        &mut self,
        exception: CpuException,
        current_pc: u32,
        in_delay_slot: bool,
    ) {
    }
}

#[derive(Debug, Default)]
pub struct StatusRegister {
    pub isolate_cache: bool,
    pub boot_exception_vectors: bool,

    // Redundant for now
    pub raw: u32,
}

impl StatusRegister {
    const BOOT_EXCEPTION_VECTOR: u32 = 0xBFC0_0180;
    const EXCEPTION_VECTOR: u32 = 0x8000_0080;

    pub fn read(&self) -> u32 {
        self.raw // Redundant for now
            | (u32::from(self.boot_exception_vectors) << 22)
            | (u32::from(self.isolate_cache) << 16)
    }

    pub fn write(&mut self, value: u32) {
        self.boot_exception_vectors = value & (1 << 22) != 0;
        self.isolate_cache = value & (1 << 16) != 0;

        self.raw = value;
    }

    pub fn handler_address(&self) -> u32 {
        if self.boot_exception_vectors {
            Self::BOOT_EXCEPTION_VECTOR
        } else {
            Self::EXCEPTION_VECTOR
        }
    }
}

#[derive(Debug, Default)]
pub struct CauseRegister {
    // raw: u32,
    /// BD
    pub in_branch_delay: bool,

    /// Undocumented: when BD=1, Branch Condition (0=False)
    pub branch_condition: bool,

    /// CD
    pub coprocessor_error: u8,

    /// IP
    pub interrupts_pending: u8,

    /// EXECODE
    pub exception_code: CpuException,
}

impl From<u32> for CauseRegister {
    fn from(value: u32) -> Self {
        Self {
            in_branch_delay: (value & (1 << 31)) != 0,
            branch_condition: (value & (1 << 30)) != 0,
            coprocessor_error: ((value >> 28) & 0b11) as u8,
            interrupts_pending: (value >> 8) as u8,
            exception_code: CpuException::from_bits((value >> 2) & 0x1F),
        }
    }
}

impl CauseRegister {
    pub fn read(&self) -> u32 {
        (u32::from(self.in_branch_delay) << 31)
            | (u32::from(self.branch_condition) << 30)
            | (u32::from(self.coprocessor_error) << 28)
            | (u32::from(self.interrupts_pending) << 8)
            | ((self.exception_code.to_cause()) << 2)
    }

    pub fn write(&mut self, value: u32) {
        // self.in_branch_delay = (value & (1 << 31)) != 0;
        self.interrupts_pending = (self.interrupts_pending & 0xFC) | (((value >> 8) & 0b11) as u8);
        // self.exception = CpuException::from_bits((value >> 2) & 0x1F);
    }
}

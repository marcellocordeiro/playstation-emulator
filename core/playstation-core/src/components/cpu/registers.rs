use crate::components::cpu::{cop0::Cop0, instruction::RegisterIndex};

/// | Name    | Alias  | Common Usage                                            |
/// | ------- | ------ | ------------------------------------------------------- |
/// | (R0)    | zero   | Constant (always 0) (this one isn't a real register)    |
/// | R1      | at     | Assembler temporary (destroyed by some pseudo opcodes!) |
/// | R2-R3   | v0-v1  | Subroutine return values, may be changed by subroutines |
/// | R4-R7   | a0-a3  | Subroutine arguments, may be changed by subroutines     |
/// | R8-R15  | t0-t7  | Temporaries, may be changed by subroutines              |
/// | R16-R23 | s0-s7  | Static variables, must be saved by subs                 |
/// | R24-R25 | t8-t9  | Temporaries, may be changed by subroutines              |
/// | R26-R27 | k0-k1  | Reserved for kernel (destroyed by some IRQ handlers!)   |
/// | R28     | gp     | Global pointer (rarely used)                            |
/// | R29     | sp     | Stack pointer                                           |
/// | R30     | fp(s8) | Frame Pointer, or 9th Static variable, must be saved    |
/// | R31     | ra     | Return address (used so by JAL,BLTZAL,BGEZAL opcodes)   |
/// | -       | pc     | Program counter                                         |
/// | -       | hi,lo  | Multiply/divide results, may be changed by subroutines  |
#[derive(Debug, Default)]
pub struct Registers {
    pub r: [u32; 32],

    pub pc: u32,
    pub hi: u32,
    pub lo: u32,

    pub delayed_load: Option<(RegisterIndex, u32)>,
    pub delayed_load_next: Option<(RegisterIndex, u32)>,

    pub delayed_branch: Option<(u32, bool)>,

    pub cop0: Cop0,
}

impl Registers {
    #[must_use]
    pub fn get_r(&self, index: RegisterIndex) -> u32 {
        self.r[index.0 as usize]
    }

    /// For LWL and LWR
    #[must_use]
    pub fn get_r_with_delayed_load(&self, index: RegisterIndex) -> u32 {
        if let Some((slot_index, value)) = self.delayed_load
            && (slot_index == index)
        {
            value
        } else {
            self.get_r(index)
        }
    }

    pub fn set_r(&mut self, index: RegisterIndex, value: u32) {
        // $zero
        if index.0 == 0 {
            return;
        }

        self.r[index.0 as usize] = value;

        if let Some(load) = self.delayed_load
            && index == load.0
        {
            self.delayed_load = None;
        }
    }

    pub fn set_r_delayed(&mut self, index: RegisterIndex, value: u32) {
        self.delayed_load_next = Some((index, value));

        if let Some(load) = self.delayed_load
            && index == load.0
        {
            self.delayed_load = None;
        }
    }

    pub fn process_load_delay(&mut self) {
        if let Some((index, value)) = self.delayed_load.take()
            && index.0 != 0
        {
            self.r[index.0 as usize] = value;
        }

        self.delayed_load = self.delayed_load_next.take();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delay() {
        let mut registers = Registers {
            delayed_load: Some((RegisterIndex(2), 0xFFFF)),
            ..Default::default()
        };

        assert_eq!(registers.delayed_load, Some((RegisterIndex(2), 0xFFFF)));
        registers.set_r(RegisterIndex(1), 0x1111);
        assert_eq!(registers.delayed_load, Some((RegisterIndex(2), 0xFFFF)));
        registers.set_r(RegisterIndex(2), 0x1111);
        assert_eq!(registers.delayed_load, None);
    }
}

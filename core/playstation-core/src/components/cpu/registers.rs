use crate::components::cpu::instruction::RegisterIndex;

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
    r: [u32; 32],

    pub pc: u32,
    pub hi: u32,
    pub lo: u32,
}

impl Registers {
    pub fn get_r(&self, index: RegisterIndex) -> u32 {
        self.r[index.0 as usize]
    }

    pub fn set_r(&mut self, index: RegisterIndex, value: u32) {
        // $zero
        if index.0 == 0 {
            return;
        }

        self.r[index.0 as usize] = value;
    }
}

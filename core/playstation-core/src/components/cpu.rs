use crate::components::{
    cpu::{instruction::Instruction, registers::Registers},
    memory::Memory,
};

pub struct Cpu {
    regs: Registers,
    memory: Memory,

    next_instruction: Instruction,

    /// COP0 register 12: Status Register
    sr: u32,
}

impl Cpu {
    pub fn new(memory: Memory) -> Self {
        let mut regs = Registers::default();
        regs.pc = 0xBFC0_0000; // Beginning of the bios

        Self {
            regs,
            memory,
            next_instruction: Instruction(0x00),
            sr: 0,
        }
    }

    pub fn run_next_instruction(&mut self) {
        let pc = self.regs.pc;

        // Delayed instruction
        let instruction = self.next_instruction;
        self.next_instruction = Instruction(self.load_dword(pc));

        self.regs.pc = self.regs.pc.wrapping_add(4);

        self.run_instruction(instruction);
    }

    fn load_dword(&self, address: u32) -> u32 {
        self.memory.load_dword(address)
    }

    fn branch(&mut self, offset: u32) {
        // PC is always aligned to 32 bits
        let offset = offset << 2;

        self.regs.pc = self.regs.pc.wrapping_add(offset).wrapping_sub(4); // run_next_instruction eagerly advances PC
    }
}

mod instruction;
mod instructions;
mod registers;

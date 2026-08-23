use crate::components::{
    cpu::{instruction::Instruction, registers::Registers},
    memory::Memory,
};

pub struct Cpu {
    regs: Registers,
    memory: Memory,

    next_instruction: Instruction,
}

impl Cpu {
    pub fn new(memory: Memory) -> Self {
        let mut regs = Registers::default();
        regs.pc = 0xBFC0_0000; // Beginning of the bios

        Self {
            regs,
            memory,
            next_instruction: Instruction(0x00),
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
}

mod instruction;
mod instructions;
mod registers;

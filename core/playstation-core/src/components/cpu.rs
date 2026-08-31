use crate::components::{
    cpu::{instruction::Instruction, registers::Registers},
    memory::MemoryInterface,
};

pub struct Cpu<Mem: MemoryInterface> {
    regs: Registers,
    memory: Mem,

    /// COP0 register 12: Status Register
    sr: u32,
}

impl<Mem: MemoryInterface> Cpu<Mem> {
    pub fn new(memory: Mem) -> Self {
        let mut regs = Registers::default();
        regs.pc = 0xBFC0_0000; // Beginning of the bios

        Self {
            regs,
            memory,
            sr: 0,
        }
    }

    pub fn run_next_instruction(&mut self) {
        let pc = self.regs.pc;

        let instruction = Instruction(self.load_word(pc));

        let (next_pc, _in_delay_slot) = match self.regs.delayed_branch.take() {
            Some((address, true)) => (address, true),
            Some((_, false)) | None => (self.regs.pc.wrapping_add(4), false),
        };

        self.regs.pc = next_pc;

        self.run_instruction(instruction);

        self.regs.process_load_delay();
    }

    fn load_word(&self, address: u32) -> u32 {
        self.memory.load_word(address)
    }

    fn branch(&mut self, offset: u32, take: bool) {
        // PC is always aligned to 32 bits
        let offset = offset << 2;
        let address = self.regs.pc.wrapping_add(offset);

        self.regs.delayed_branch = Some((address, take));
    }

    fn jump(&mut self, address: u32) {
        self.regs.delayed_branch = Some((address, true));
    }
}

mod instruction;
mod instructions;
mod registers;

#[cfg(test)]
mod tests;

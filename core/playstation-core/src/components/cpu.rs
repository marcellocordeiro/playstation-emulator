use crate::components::{
    cpu::{instruction::Instruction, instructions::CpuException, registers::Registers},
    memory::MemoryInterface,
};

pub struct Cpu<Mem: MemoryInterface> {
    regs: Registers,
    memory: Mem,
}

impl<Mem: MemoryInterface> Cpu<Mem> {
    pub fn new(memory: Mem) -> Self {
        let mut regs = Registers::default();
        regs.pc = 0xBFC0_0000; // Beginning of the bios

        Self { regs, memory }
    }

    pub fn run_next_instruction(&mut self) {
        let current_pc = self.regs.pc;

        let (next_pc, in_delay_slot) = match self.regs.delayed_branch.take() {
            Some((address, true)) => (address, true),
            Some((_, false)) | None => (self.regs.pc.wrapping_add(4), false),
        };

        if current_pc % 4 != 0 {
            // Address error on opcode fetch
            self.handle_exception(CpuException::LoadAddressError, current_pc, in_delay_slot);
            self.regs.process_load_delay();

            return;
        }

        let instruction = Instruction(self.load_word(current_pc));

        self.regs.pc = next_pc;

        let result = self.run_instruction(instruction);

        if let Err(exception) = result {
            self.handle_exception(exception, current_pc, in_delay_slot);
        }

        self.regs.process_load_delay();
    }

    fn handle_exception(&mut self, exception: CpuException, current_pc: u32, in_delay_slot: bool) {
        let handler = self.regs.cop0.sr.handler_address();

        let mode = self.regs.cop0.sr.raw & 0x3F;
        self.regs.cop0.sr.raw &= !0x3F;
        self.regs.cop0.sr.raw |= (mode << 2) & 0x3F;

        self.regs.cop0.cause.in_branch_delay = in_delay_slot;
        self.regs.cop0.cause.exception_code = exception;

        self.regs.cop0.epc = if in_delay_slot {
            current_pc.wrapping_sub(4)
        } else {
            current_pc
        };

        self.regs.pc = handler;
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

mod cop0;
mod instruction;
mod instructions;
mod registers;

#[cfg(test)]
mod tests;

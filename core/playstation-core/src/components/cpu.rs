use std::time;

use crate::{
    components::{
        cpu::{instruction::Instruction, instructions::CpuException, registers::Registers},
        memory::MemoryInterface,
    },
    psx_executable::PsxExecutable,
};

pub struct Cpu<Mem: MemoryInterface> {
    pub regs: Registers,
    pub memory: Mem,
}

impl<Mem: MemoryInterface> Cpu<Mem> {
    #[must_use]
    pub fn new(memory: Mem) -> Self {
        let regs = Registers {
            pc: 0xBFC0_0000, // Beginning of the bios
            ..Default::default()
        };

        Self { regs, memory }
    }

    pub fn run_next_instruction(&mut self) {
        let current_pc = self.regs.pc;

        self.check_for_tty_output();

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

        let instruction = self.fetch_instruction(current_pc);

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

    fn branch(&mut self, offset: u32, take: bool) {
        // // PC is always aligned to 32 bits
        // let offset = offset << 2;
        let address = self.regs.pc.wrapping_add(offset);

        self.regs.delayed_branch = Some((address, take));
    }

    fn jump(&mut self, address: u32) {
        self.regs.delayed_branch = Some((address, true));
    }

    fn fetch_instruction(&self, address: u32) -> Instruction {
        Instruction(self.memory.fetch_instruction(address))
    }

    // Testing

    fn check_for_tty_output(&self) {
        let pc = self.regs.pc & 0x1FFF_FFFF;

        if (pc == 0xA0 && self.regs.r[9] == 0x3C) || (pc == 0xB0 && self.regs.r[9] == 0x3D) {
            let ch = self.regs.r[4] as u8 as char;
            print!("{ch}");
        }
    }

    pub fn sideload_amidogs(&mut self) {
        let exe = include_bytes!("../../../../roms/psxtest_cpu.exe");

        // Wait for the BIOS to jump to the shell
        while self.regs.pc != 0x8003_0000 {
            self.run_next_instruction();
        }

        PsxExecutable::apply(exe, &mut self.regs, self.memory.ram_mut());

        std::thread::sleep(time::Duration::from_secs(1));
    }
}

mod cop0;
pub mod decoded_instruction;
pub mod instruction;
mod instructions;
pub mod registers;

#[cfg(test)]
mod tests;

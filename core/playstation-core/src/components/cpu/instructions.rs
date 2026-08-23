use tracing::info;

use crate::components::cpu::{Cpu, instruction::Instruction};

impl Cpu {
    pub fn run_instruction(&mut self, instruction: Instruction) {
        info!("Executing {instruction:?}");

        match instruction.primary() {
            // Secondary
            0x00 if instruction.secondary() == 0x00 => self.sll(instruction),

            0x00 if instruction.secondary() == 0x02 => {
                // SRL
                panic!("Invalid/unimplemented instruction: {instruction:?}",)
            }

            0x00 if instruction.secondary() == 0x03 => {
                // SRA
                panic!("Invalid/unimplemented instruction: {instruction:?}",)
            }

            0x00 if instruction.secondary() == 0x04 => {
                // SLLV
                panic!("Invalid/unimplemented instruction: {instruction:?}",)
            }

            0x00 if instruction.secondary() == 0x06 => {
                // SRLV
                panic!("Invalid/unimplemented instruction: {instruction:?}",)
            }

            0x00 if instruction.secondary() == 0x07 => {
                // SRAV
                panic!("Invalid/unimplemented instruction: {instruction:?}",)
            }

            0x00 if instruction.secondary() == 0x08 => {
                // JR
                panic!("Invalid/unimplemented instruction: {instruction:?}",)
            }

            0x00 if instruction.secondary() == 0x09 => {
                // JALR
                panic!("Invalid/unimplemented instruction: {instruction:?}",)
            }

            0x00 if instruction.secondary() == 0x0C => {
                // SYSCALL
                panic!("Invalid/unimplemented instruction: {instruction:?}",)
            }

            0x00 if instruction.secondary() == 0x0D => {
                // BREAK
                panic!("Invalid/unimplemented instruction: {instruction:?}",)
            }

            0x00 if instruction.secondary() == 0x10 => {
                // MFHI
                panic!("Invalid/unimplemented instruction: {instruction:?}",)
            }

            0x00 if instruction.secondary() == 0x11 => {
                // MTHI
                panic!("Invalid/unimplemented instruction: {instruction:?}",)
            }

            0x00 if instruction.secondary() == 0x12 => {
                // MFLO
                panic!("Invalid/unimplemented instruction: {instruction:?}",)
            }

            0x00 if instruction.secondary() == 0x13 => {
                // MTLO
                panic!("Invalid/unimplemented instruction: {instruction:?}",)
            }

            0x00 if instruction.secondary() == 0x18 => {
                // MULT
                panic!("Invalid/unimplemented instruction: {instruction:?}",)
            }

            0x00 if instruction.secondary() == 0x19 => {
                // MULTU
                panic!("Invalid/unimplemented instruction: {instruction:?}",)
            }

            0x00 if instruction.secondary() == 0x1A => {
                // DIV
                panic!("Invalid/unimplemented instruction: {instruction:?}",)
            }

            0x00 if instruction.secondary() == 0x1B => {
                // DIVU
                panic!("Invalid/unimplemented instruction: {instruction:?}",)
            }

            0x00 if instruction.secondary() == 0x20 => {
                // ADD
                panic!("Invalid/unimplemented instruction: {instruction:?}",)
            }

            0x00 if instruction.secondary() == 0x21 => {
                // ADDU
                panic!("Invalid/unimplemented instruction: {instruction:?}",)
            }

            0x00 if instruction.secondary() == 0x22 => {
                // SUB
                panic!("Invalid/unimplemented instruction: {instruction:?}",)
            }

            0x00 if instruction.secondary() == 0x23 => {
                // SUBU
                panic!("Invalid/unimplemented instruction: {instruction:?}",)
            }

            0x00 if instruction.secondary() == 0x24 => {
                // AND
                panic!("Invalid/unimplemented instruction: {instruction:?}",)
            }

            0x00 if instruction.secondary() == 0x25 => self.or(instruction),

            0x00 if instruction.secondary() == 0x26 => {
                // XOR
                panic!("Invalid/unimplemented instruction: {instruction:?}",)
            }

            0x00 if instruction.secondary() == 0x27 => {
                // NOR
                panic!("Invalid/unimplemented instruction: {instruction:?}",)
            }

            0x00 if instruction.secondary() == 0x2A => {
                // SLT
                panic!("Invalid/unimplemented instruction: {instruction:?}",)
            }

            0x00 if instruction.secondary() == 0x2B => {
                // SLTU
                panic!("Invalid/unimplemented instruction: {instruction:?}",)
            }

            // Primary
            0x01 => {
                // BcondZ
                panic!("Invalid/unimplemented instruction: {instruction:?}",)
            }

            0x02 => self.j(instruction),

            0x03 => {
                // JAL
                panic!("Invalid/unimplemented instruction: {instruction:?}",)
            }

            0x04 => {
                // BEQ
                panic!("Invalid/unimplemented instruction: {instruction:?}",)
            }

            0x05 => {
                // BNE
                panic!("Invalid/unimplemented instruction: {instruction:?}",)
            }

            0x06 => {
                // BLEZ
                panic!("Invalid/unimplemented instruction: {instruction:?}",)
            }

            0x07 => {
                // BGTZ
                panic!("Invalid/unimplemented instruction: {instruction:?}",)
            }

            0x08 => {
                // ADDI
                panic!("Invalid/unimplemented instruction: {instruction:?}",)
            }

            0x09 => self.addiu(instruction),

            0x0A => {
                // SLTI
                panic!("Invalid/unimplemented instruction: {instruction:?}",)
            }

            0x0B => {
                // SLTIU
                panic!("Invalid/unimplemented instruction: {instruction:?}",)
            }

            0x0C => {
                // ANDI
                panic!("Invalid/unimplemented instruction: {instruction:?}",)
            }

            0x0D => self.ori(instruction),

            0x0E => {
                // XORI
                panic!("Invalid/unimplemented instruction: {instruction:?}",)
            }

            0x0F => self.lui(instruction),

            0x10 => {
                // COP0
                panic!("Invalid/unimplemented instruction: {instruction:?}",)
            }

            0x11 => {
                // COP1
                panic!("Invalid/unimplemented instruction: {instruction:?}",)
            }

            0x12 => {
                // COP2
                panic!("Invalid/unimplemented instruction: {instruction:?}",)
            }

            0x20 => {
                // LB
                panic!("Invalid/unimplemented instruction: {instruction:?}",)
            }

            0x21 => {
                // LH
                panic!("Invalid/unimplemented instruction: {instruction:?}",)
            }

            0x22 => {
                // LWL
                panic!("Invalid/unimplemented instruction: {instruction:?}",)
            }

            0x23 => {
                // LW
                panic!("Invalid/unimplemented instruction: {instruction:?}",)
            }

            0x24 => {
                // LBU
                panic!("Invalid/unimplemented instruction: {instruction:?}",)
            }

            0x25 => {
                // LHU
                panic!("Invalid/unimplemented instruction: {instruction:?}",)
            }

            0x26 => {
                // LWR
                panic!("Invalid/unimplemented instruction: {instruction:?}",)
            }

            0x28 => {
                // SB
                panic!("Invalid/unimplemented instruction: {instruction:?}",)
            }

            0x29 => {
                // SH
                panic!("Invalid/unimplemented instruction: {instruction:?}",)
            }

            0x2A => {
                // SWL
                panic!("Invalid/unimplemented instruction: {instruction:?}",)
            }

            0x2B => self.sw(instruction),

            0x2E => {
                // SWR
                panic!("Invalid/unimplemented instruction: {instruction:?}",)
            }

            0x30 => {
                // LWC0
                panic!("Invalid/unimplemented instruction: {instruction:?}",)
            }

            0x31 => {
                // LWC1
                panic!("Invalid/unimplemented instruction: {instruction:?}",)
            }

            0x32 => {
                // LWC2
                panic!("Invalid/unimplemented instruction: {instruction:?}",)
            }

            0x33 => {
                // LWC3
                panic!("Invalid/unimplemented instruction: {instruction:?}",)
            }

            0x38 => {
                // SWC0
                panic!("Invalid/unimplemented instruction: {instruction:?}",)
            }

            0x39 => {
                // SWC1
                panic!("Invalid/unimplemented instruction: {instruction:?}",)
            }

            0x3A => {
                // SWC2
                panic!("Invalid/unimplemented instruction: {instruction:?}",)
            }

            0x3B => {
                // SWC3
                panic!("Invalid/unimplemented instruction: {instruction:?}",)
            }

            _ => {
                panic!("Invalid/unimplemented instruction: {instruction:?}",)
            }
        }
    }
}

impl Cpu {
    // Primary

    /// Load Upper Intermediate
    fn lui(&mut self, instruction: Instruction) {
        let imm = instruction.imm();
        let rt = instruction.rt();

        let value = imm << 16;

        self.regs.set_r(rt, value);
    }

    fn ori(&mut self, instruction: Instruction) {
        let imm = instruction.imm();
        let rt = instruction.rt();
        let rs = instruction.rs();

        let value = self.regs.get_r(rs) | imm;
        self.regs.set_r(rt, value);
    }

    /// Store Word
    fn sw(&mut self, instruction: Instruction) {
        let imm = instruction.imm_sign_extended();
        let rt = instruction.rt();
        let rs = instruction.rs();

        let address = self.regs.get_r(rs).wrapping_add(imm);
        let value = self.regs.get_r(rt);

        self.memory.store_dword(address, value);
    }

    /// Add Immediate Unsigned
    fn addiu(&mut self, instruction: Instruction) {
        let imm = instruction.imm_sign_extended();
        let rt = instruction.rt();
        let rs = instruction.rd();

        let value = self.regs.get_r(rs).wrapping_add(imm);

        self.regs.set_r(rt, value);
    }

    /// Jump
    fn j(&mut self, instruction: Instruction) {
        let imm = instruction.imm_jump();

        self.regs.pc = (self.regs.pc & 0xF000_0000) | (imm << 2);
    }
}

impl Cpu {
    // Secondary

    /// Shift Left Logical
    fn sll(&mut self, instruction: Instruction) {
        let imm = instruction.shift_imm();
        let rt = instruction.rt();
        let rd = instruction.rd();

        let value = self.regs.get_r(rt) << imm;

        self.regs.set_r(rd, value);
    }

    /// Bitwise Or
    fn or(&mut self, instruction: Instruction) {
        let rs = instruction.rs();
        let rt = instruction.rt();
        let rd = instruction.rd();

        let value = self.regs.get_r(rs) | self.regs.get_r(rt);

        self.regs.set_r(rd, value);
    }
}

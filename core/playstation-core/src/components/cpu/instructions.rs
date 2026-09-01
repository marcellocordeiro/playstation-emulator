use tracing::info;

use crate::components::{
    cpu::{
        Cpu,
        instruction::{Instruction, RegisterIndex},
    },
    memory::MemoryInterface,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum CpuException {
    #[default]
    Interrupt,
    LoadAddressError,
    StoreAddressError,
    Syscall,
    Breakpoint,
    ArithmeticOverflow,

    Other(u32),
}

impl CpuException {
    pub fn from_bits(value: u32) -> Self {
        match value {
            0x0 => Self::Interrupt,
            0x4 => Self::LoadAddressError,
            0x5 => Self::StoreAddressError,
            0x8 => Self::Syscall,
            0x9 => Self::Breakpoint,
            0xC => Self::ArithmeticOverflow,

            other => Self::Other(other),
        }
    }

    pub fn to_cause(self) -> u32 {
        match self {
            Self::Interrupt => 0x0,
            Self::LoadAddressError => 0x4,
            Self::StoreAddressError => 0x5,
            Self::Syscall => 0x8,
            Self::Breakpoint => 0x9,
            Self::ArithmeticOverflow => 0xC,

            Self::Other(value) => value,
        }
    }
}

type CpuExecutionResult<T> = Result<T, CpuException>;

impl<Mem: MemoryInterface> Cpu<Mem> {
    pub fn run_instruction(&mut self, instruction: Instruction) -> CpuExecutionResult<()> {
        //info!("Executing {instruction:?}");

        match instruction.primary() {
            // Secondary
            0x00 if instruction.secondary() == 0x00 => self.sll(instruction),
            0x00 if instruction.secondary() == 0x02 => self.srl(instruction),
            0x00 if instruction.secondary() == 0x03 => self.sra(instruction),
            0x00 if instruction.secondary() == 0x04 => self.sllv(instruction),
            0x00 if instruction.secondary() == 0x06 => self.srlv(instruction),
            0x00 if instruction.secondary() == 0x07 => self.srav(instruction),
            0x00 if instruction.secondary() == 0x08 => self.jr(instruction),
            0x00 if instruction.secondary() == 0x09 => self.jalr(instruction),
            0x00 if instruction.secondary() == 0x0C => self.syscall(instruction)?,
            0x00 if instruction.secondary() == 0x0D => self.r#break(instruction)?,
            0x00 if instruction.secondary() == 0x10 => self.mfhi(instruction),
            0x00 if instruction.secondary() == 0x11 => self.mthi(instruction),
            0x00 if instruction.secondary() == 0x12 => self.mflo(instruction),
            0x00 if instruction.secondary() == 0x13 => self.mtlo(instruction),
            0x00 if instruction.secondary() == 0x18 => self.mult(instruction),
            0x00 if instruction.secondary() == 0x19 => self.multu(instruction),
            0x00 if instruction.secondary() == 0x1A => self.div(instruction),
            0x00 if instruction.secondary() == 0x1B => self.divu(instruction),
            0x00 if instruction.secondary() == 0x20 => self.add(instruction)?,
            0x00 if instruction.secondary() == 0x21 => self.addu(instruction),
            0x00 if instruction.secondary() == 0x22 => self.sub(instruction)?,
            0x00 if instruction.secondary() == 0x23 => self.subu(instruction),
            0x00 if instruction.secondary() == 0x24 => self.and(instruction),
            0x00 if instruction.secondary() == 0x25 => self.or(instruction),
            0x00 if instruction.secondary() == 0x26 => self.xor(instruction),
            0x00 if instruction.secondary() == 0x27 => self.nor(instruction),
            0x00 if instruction.secondary() == 0x2A => self.slt(instruction),
            0x00 if instruction.secondary() == 0x2B => self.sltu(instruction),

            // Primary
            0x01 => self.b_cond_z(instruction),
            0x02 => self.j(instruction),
            0x03 => self.jal(instruction),
            0x04 => self.beq(instruction),
            0x05 => self.bne(instruction),
            0x06 => self.blez(instruction),
            0x07 => self.bgtz(instruction),
            0x08 => self.addi(instruction)?,
            0x09 => self.addiu(instruction),
            0x0A => self.slti(instruction),
            0x0B => self.sltiu(instruction),
            0x0C => self.andi(instruction),
            0x0D => self.ori(instruction),
            0x0E => self.xori(instruction),
            0x0F => self.lui(instruction),
            0x10 => self.cop0(instruction),
            0x11 => self.cop1(instruction),
            0x12 => self.cop2(instruction),
            0x13 => self.cop3(instruction),
            0x20 => self.lb(instruction),
            0x21 => self.lh(instruction)?,
            0x22 => self.lwl(instruction),
            0x23 => self.lw(instruction)?,
            0x24 => self.lbu(instruction),
            0x25 => self.lhu(instruction)?,
            0x26 => self.lwr(instruction),
            0x28 => self.sb(instruction),
            0x29 => self.sh(instruction)?,
            0x2A => self.swl(instruction),
            0x2B => self.sw(instruction)?,
            0x2E => self.swr(instruction),
            0x30 => self.lwc0(instruction),
            0x31 => self.lwc1(instruction),
            0x32 => self.lwc2(instruction),
            0x33 => self.lwc3(instruction),
            0x38 => self.swc0(instruction),
            0x39 => self.swc1(instruction),
            0x3A => self.swc2(instruction),
            0x3B => self.swc3(instruction),
            _ => panic!("Invalid instruction: {instruction:?}"),
        }

        Ok(())
    }
}

impl<Mem: MemoryInterface> Cpu<Mem> {
    // Primary

    /// BLTZ, BLTZAL, BGEZ and BGEZAL
    fn b_cond_z(&mut self, instruction: Instruction) {
        match (instruction.0 >> 16) & 0x1F {
            0b10000 => self.bltzal(instruction),
            0b10001 => self.bgezal(instruction),

            // Undocumented dupes
            _ => {
                let is_bgez = ((instruction.0 >> 16) & 1) != 0;

                if is_bgez {
                    self.bgez(instruction);
                } else {
                    self.bltz(instruction);
                }
            }
        }
    }

    /// Jump
    fn j(&mut self, instruction: Instruction) {
        let imm = instruction.imm_jump();

        let address = (self.regs.pc & 0xF000_0000) | imm;

        self.jump(address);
    }

    /// Jump And Link
    fn jal(&mut self, instruction: Instruction) {
        let ra = self.regs.pc.wrapping_add(4);
        let index = RegisterIndex(31);

        self.regs.set_r(index, ra);

        self.j(instruction);
    }

    /// Branch if Equal
    fn beq(&mut self, instruction: Instruction) {
        let offset = instruction.imm_sign_extended();
        let rs = instruction.rs();
        let rt = instruction.rt();

        let take = self.regs.get_r(rs) == self.regs.get_r(rt);

        self.branch(offset, take);
    }

    /// Branch if Not Equal
    fn bne(&mut self, instruction: Instruction) {
        let offset = instruction.imm_sign_extended();
        let rs = instruction.rs();
        let rt = instruction.rt();

        let take = self.regs.get_r(rs) != self.regs.get_r(rt);

        self.branch(offset, take);
    }

    /// Branch if Less than or Equal to Zero
    fn blez(&mut self, instruction: Instruction) {
        let offset = instruction.imm_sign_extended();
        let rs = instruction.rs();

        let s = self.regs.get_r(rs) as i32;

        let take = s <= 0;

        self.branch(offset, take);
    }

    /// Branch if Greater than Zero
    fn bgtz(&mut self, instruction: Instruction) {
        let offset = instruction.imm_sign_extended();
        let rs = instruction.rs();

        let s = self.regs.get_r(rs) as i32;

        let take = s > 0;

        self.branch(offset, take);
    }

    /// Add Immediate
    fn addi(&mut self, instruction: Instruction) -> CpuExecutionResult<()> {
        let imm = instruction.imm_sign_extended() as i32;
        let rt = instruction.rt();
        let rs = instruction.rs();

        let value = match (self.regs.get_r(rs) as i32).checked_add(imm) {
            Some(value) => value as u32,
            None => return Err(CpuException::ArithmeticOverflow),
        };

        self.regs.set_r(rt, value);

        Ok(())
    }

    /// Add Immediate Unsigned
    fn addiu(&mut self, instruction: Instruction) {
        let imm = instruction.imm_sign_extended();
        let rt = instruction.rt();
        let rs = instruction.rs();

        let value = self.regs.get_r(rs).wrapping_add(imm);

        self.regs.set_r(rt, value);
    }

    /// Set if Less Than Immediate
    fn slti(&mut self, instruction: Instruction) {
        let imm = instruction.imm_sign_extended() as i32;
        let rs = instruction.rs();
        let rt = instruction.rt();

        let result = (self.regs.get_r(rs) as i32) < imm;

        self.regs.set_r(rt, result as u32);
    }

    /// Set if Less Than Immediate Unsigned
    fn sltiu(&mut self, instruction: Instruction) {
        let imm = instruction.imm_sign_extended();
        let rs = instruction.rs();
        let rt = instruction.rt();

        let result = self.regs.get_r(rs) < imm;

        self.regs.set_r(rt, result as u32);
    }

    /// Bitwise AND Immediate
    fn andi(&mut self, instruction: Instruction) {
        let imm = instruction.imm();
        let rt = instruction.rt();
        let rs = instruction.rs();

        let value = self.regs.get_r(rs) & imm;
        self.regs.set_r(rt, value);
    }

    /// Bitwise OR Immediate
    fn ori(&mut self, instruction: Instruction) {
        let imm = instruction.imm();
        let rt = instruction.rt();
        let rs = instruction.rs();

        let value = self.regs.get_r(rs) | imm;
        self.regs.set_r(rt, value);
    }

    /// Bitwise XOR immediate
    fn xori(&mut self, instruction: Instruction) {
        let imm = instruction.imm();
        let rt = instruction.rt();
        let rs = instruction.rs();

        let value = self.regs.get_r(rs) ^ imm;
        self.regs.set_r(rt, value);
    }

    /// Load Upper Intermediate
    fn lui(&mut self, instruction: Instruction) {
        let imm = instruction.imm();
        let rt = instruction.rt();

        let value = imm << 16;

        self.regs.set_r(rt, value);
    }

    fn cop0(&mut self, instruction: Instruction) {
        match instruction.cop_opcode() {
            0b00000 => self.mfc0(instruction),
            0b00100 => self.mtc0(instruction),
            0b10000 => self.rfe(instruction),

            _ => {
                unimplemented!(
                    "Unimplemented COP0 instruction: {instruction:?}, cop_opcode = {:08X}",
                    instruction.cop_opcode()
                )
            }
        }
    }

    fn cop1(&mut self, instruction: Instruction) {
        // COP1
        unimplemented!("{instruction:?}");
    }

    fn cop2(&mut self, instruction: Instruction) {
        // COP2
        unimplemented!("{instruction:?}");
    }

    fn cop3(&mut self, instruction: Instruction) {
        // COP3
        unimplemented!("{instruction:?}");
    }

    /// Load Byte
    fn lb(&mut self, instruction: Instruction) {
        if self.regs.cop0.sr.isolate_cache {
            // Cache is isolated, ignore writes
            info!("Ignoring store while cache is isolated");
            return;
        }

        let imm = instruction.imm_sign_extended();
        let rt = instruction.rt();
        let rs = instruction.rs();

        let address = self.regs.get_r(rs).wrapping_add(imm);
        let value = self.memory.load_byte(address) as i8;

        // Load delay slot
        self.regs.set_r_delayed(rt, value as u32);
    }

    /// Load Halfword
    fn lh(&mut self, instruction: Instruction) -> CpuExecutionResult<()> {
        let imm = instruction.imm_sign_extended();
        let rt = instruction.rt();
        let rs = instruction.rs();

        let address = self.regs.get_r(rs).wrapping_add(imm);

        if address % 2 == 0 {
            let value = self.memory.load_halfword(address) as i16;

            // Load delay slot
            self.regs.set_r_delayed(rt, value as u32);
        } else {
            return Err(CpuException::LoadAddressError);
        }

        Ok(())
    }

    fn lwl(&mut self, instruction: Instruction) {
        // LWL
        unimplemented!("{instruction:?}");
    }

    /// Load Word
    fn lw(&mut self, instruction: Instruction) -> CpuExecutionResult<()> {
        if self.regs.cop0.sr.isolate_cache {
            // Cache is isolated, ignore writes
            info!("Ignoring store while cache is isolated");
            return Ok(());
        }

        let imm = instruction.imm_sign_extended();
        let rt = instruction.rt();
        let rs = instruction.rs();

        let address = self.regs.get_r(rs).wrapping_add(imm);

        if address % 4 == 0 {
            let value = self.load_word(address);

            // Load delay slot
            self.regs.set_r_delayed(rt, value);
        } else {
            return Err(CpuException::LoadAddressError);
        }

        Ok(())
    }

    /// Load Byte Unsigned
    fn lbu(&mut self, instruction: Instruction) {
        if self.regs.cop0.sr.isolate_cache {
            // Cache is isolated, ignore writes
            info!("Ignoring store while cache is isolated");
            return;
        }

        let imm = instruction.imm_sign_extended();
        let rt = instruction.rt();
        let rs = instruction.rs();

        let address = self.regs.get_r(rs).wrapping_add(imm);
        let value = self.memory.load_byte(address);

        // Load delay slot
        self.regs.set_r_delayed(rt, value as u32);
    }

    /// Load Halfword Unsigned
    fn lhu(&mut self, instruction: Instruction) -> CpuExecutionResult<()> {
        let imm = instruction.imm_sign_extended();
        let rt = instruction.rt();
        let rs = instruction.rs();

        let address = self.regs.get_r(rs).wrapping_add(imm);

        if address % 2 == 0 {
            let value = self.memory.load_halfword(address);

            // Load delay slot
            self.regs.set_r_delayed(rt, value as u32);
        } else {
            return Err(CpuException::LoadAddressError);
        }

        Ok(())
    }

    fn lwr(&mut self, instruction: Instruction) {
        // LWR
        unimplemented!("{instruction:?}");
    }

    /// Store Byte
    fn sb(&mut self, instruction: Instruction) {
        if self.regs.cop0.sr.isolate_cache {
            // Cache is isolated, ignore writes
            info!("Ignoring store while cache is isolated");
            return;
        }

        let imm = instruction.imm_sign_extended();
        let rt = instruction.rt();
        let rs = instruction.rs();

        let address = self.regs.get_r(rs).wrapping_add(imm);
        let value = self.regs.get_r(rt) as u8;

        self.memory.store_byte(address, value);
    }

    /// Store Halfword
    fn sh(&mut self, instruction: Instruction) -> CpuExecutionResult<()> {
        if self.regs.cop0.sr.isolate_cache {
            // Cache is isolated, ignore writes
            info!("Ignoring store while cache is isolated");
            return Ok(());
        }

        let imm = instruction.imm_sign_extended();
        let rt = instruction.rt();
        let rs = instruction.rs();

        let address = self.regs.get_r(rs).wrapping_add(imm);

        if address % 2 == 0 {
            let value = self.regs.get_r(rt) as u16;

            self.memory.store_halfword(address, value);
        } else {
            return Err(CpuException::StoreAddressError);
        }

        Ok(())
    }

    fn swl(&mut self, instruction: Instruction) {
        // SWL
        unimplemented!("{instruction:?}");
    }

    /// Store Word
    fn sw(&mut self, instruction: Instruction) -> CpuExecutionResult<()> {
        if self.regs.cop0.sr.isolate_cache {
            // Cache is isolated, ignore writes
            info!("Ignoring store while cache is isolated");
            return Ok(());
        }

        let imm = instruction.imm_sign_extended();
        let rt = instruction.rt();
        let rs = instruction.rs();

        let address = self.regs.get_r(rs).wrapping_add(imm);

        if address % 4 == 0 {
            let value = self.regs.get_r(rt);

            self.memory.store_word(address, value);
        } else {
            return Err(CpuException::StoreAddressError);
        }

        Ok(())
    }

    fn swr(&mut self, instruction: Instruction) {
        // SWR
        unimplemented!("{instruction:?}");
    }

    fn lwc0(&mut self, instruction: Instruction) {
        // LWC0
        unimplemented!("{instruction:?}");
    }

    fn lwc1(&mut self, instruction: Instruction) {
        // LWC1
        unimplemented!("{instruction:?}");
    }

    fn lwc2(&mut self, instruction: Instruction) {
        // LWC2
        unimplemented!("{instruction:?}");
    }

    fn lwc3(&mut self, instruction: Instruction) {
        // LWC3
        unimplemented!("{instruction:?}");
    }

    fn swc0(&mut self, instruction: Instruction) {
        // SWC0
        unimplemented!("{instruction:?}");
    }

    fn swc1(&mut self, instruction: Instruction) {
        // SWC1
        unimplemented!("{instruction:?}");
    }

    fn swc2(&mut self, instruction: Instruction) {
        // SWC2
        unimplemented!("{instruction:?}");
    }

    fn swc3(&mut self, instruction: Instruction) {
        // SWC3
        unimplemented!("{instruction:?}");
    }
}

impl<Mem: MemoryInterface> Cpu<Mem> {
    // Secondary

    /// Shift Left Logical
    fn sll(&mut self, instruction: Instruction) {
        let imm = instruction.shift_imm();
        let rt = instruction.rt();
        let rd = instruction.rd();

        let value = self.regs.get_r(rt) << imm;

        self.regs.set_r(rd, value);
    }

    /// Shift Right Logical
    fn srl(&mut self, instruction: Instruction) {
        let imm = instruction.shift_imm();
        let rt = instruction.rt();
        let rd = instruction.rd();

        let value = self.regs.get_r(rt) >> imm;

        self.regs.set_r(rd, value);
    }

    /// Shift Right Arithmetic
    fn sra(&mut self, instruction: Instruction) {
        let imm = instruction.shift_imm();
        let rt = instruction.rt();
        let rd = instruction.rd();

        let result = (self.regs.get_r(rt) as i32) >> imm;

        self.regs.set_r(rd, result as u32);
    }

    /// Shift Left Logical Variable
    fn sllv(&mut self, instruction: Instruction) {
        let rs = instruction.rs();
        let rt = instruction.rt();
        let rd = instruction.rd();

        let shamnt = self.regs.get_r(rs) & 0x1F;

        let result = self.regs.get_r(rt) << shamnt;

        self.regs.set_r(rd, result);
    }

    /// Shift Right Logical Variable
    fn srlv(&mut self, instruction: Instruction) {
        let rs = instruction.rs();
        let rt = instruction.rt();
        let rd = instruction.rd();

        let shamnt = self.regs.get_r(rs) & 0x1F;

        let result = self.regs.get_r(rt) >> shamnt;

        self.regs.set_r(rd, result);
    }

    fn srav(&mut self, instruction: Instruction) {
        let rs = instruction.rs();
        let rt = instruction.rt();
        let rd = instruction.rd();

        let shamnt = self.regs.get_r(rs) & 0x1F;

        let result = (self.regs.get_r(rt) as i32) >> shamnt;

        self.regs.set_r(rd, result as u32);
    }

    /// Jump Register
    fn jr(&mut self, instruction: Instruction) {
        let rs = instruction.rs();

        let value = self.regs.get_r(rs);

        self.jump(value);
    }

    /// Jump And Link Register
    fn jalr(&mut self, instruction: Instruction) {
        let rs = instruction.rs();
        let rd = instruction.rd();

        let ra = self.regs.pc.wrapping_add(4);

        // Jump
        // Need to fetch $(rs) first in case the link writes to it
        let value = self.regs.get_r(rs);
        self.jump(value);

        // Link
        self.regs.set_r(rd, ra);
    }

    fn syscall(&mut self, _instruction: Instruction) -> CpuExecutionResult<()> {
        Err(CpuException::Syscall)
    }

    fn r#break(&mut self, _instruction: Instruction) -> CpuExecutionResult<()> {
        Err(CpuException::Breakpoint)
    }

    /// Move From HI
    fn mfhi(&mut self, instruction: Instruction) {
        let rd = instruction.rd();

        let value = self.regs.hi;

        self.regs.set_r(rd, value);
    }

    /// Move To HI
    fn mthi(&mut self, instruction: Instruction) {
        let rs = instruction.rs();

        let value = self.regs.get_r(rs);

        self.regs.hi = value;
    }

    /// Move From LO
    fn mflo(&mut self, instruction: Instruction) {
        let rd = instruction.rd();

        let value = self.regs.lo;

        self.regs.set_r(rd, value);
    }

    fn mtlo(&mut self, instruction: Instruction) {
        let rs = instruction.rs();

        let value = self.regs.get_r(rs);

        self.regs.lo = value;
    }

    /// Multiply
    fn mult(&mut self, instruction: Instruction) {
        let rs = instruction.rs();
        let rt = instruction.rt();

        let a = self.regs.get_r(rs) as i32 as i64;
        let b = self.regs.get_r(rt) as i32 as i64;

        let result = (a * b) as u64;

        self.regs.hi = (result >> 32) as u32;
        self.regs.lo = result as u32;
    }

    /// Multiply Unsigned
    fn multu(&mut self, instruction: Instruction) {
        let rs = instruction.rs();
        let rt = instruction.rt();

        let a = self.regs.get_r(rs) as u64;
        let b = self.regs.get_r(rt) as u64;

        let result = a * b;

        self.regs.hi = (result >> 32) as u32;
        self.regs.lo = result as u32;
    }

    /// Divide
    fn div(&mut self, instruction: Instruction) {
        let rs = instruction.rs();
        let rt = instruction.rt();

        let dividend = self.regs.get_r(rs) as i32;
        let divisor = self.regs.get_r(rt) as i32;

        let (quotient, remainder) = if divisor == 0 {
            (if dividend >= 0 { u32::MAX } else { 1 }, dividend as u32)
        } else if (dividend as u32) == 0x8000_0000 && divisor == -1 {
            (0x8000_0000, 0)
        } else {
            ((dividend / divisor) as u32, (dividend % divisor) as u32)
        };

        self.regs.hi = remainder;
        self.regs.lo = quotient;
    }

    /// Divide Unsigned
    fn divu(&mut self, instruction: Instruction) {
        let rs = instruction.rs();
        let rt = instruction.rt();

        let dividend = self.regs.get_r(rs);
        let divisor = self.regs.get_r(rt);

        let (quotient, remainder) = if divisor == 0 {
            (u32::MAX, dividend)
        } else {
            (dividend / divisor, dividend % divisor)
        };

        self.regs.hi = remainder;
        self.regs.lo = quotient;
    }

    /// Add
    fn add(&mut self, instruction: Instruction) -> CpuExecutionResult<()> {
        let rs = instruction.rs();
        let rt = instruction.rt();
        let rd = instruction.rd();

        let s = self.regs.get_r(rs) as i32;
        let t = self.regs.get_r(rt) as i32;

        let result = match (s).checked_add(t) {
            Some(value) => value as u32,
            None => return Err(CpuException::ArithmeticOverflow),
        };

        self.regs.set_r(rd, result);

        Ok(())
    }

    /// Add Unsigned
    fn addu(&mut self, instruction: Instruction) {
        let rs = instruction.rs();
        let rt = instruction.rt();
        let rd = instruction.rd();

        let result = self.regs.get_r(rs).wrapping_add(self.regs.get_r(rt));

        self.regs.set_r(rd, result);
    }

    /// Sub
    fn sub(&mut self, instruction: Instruction) -> CpuExecutionResult<()> {
        let rs = instruction.rs();
        let rt = instruction.rt();
        let rd = instruction.rd();

        let s = self.regs.get_r(rs) as i32;
        let t = self.regs.get_r(rt) as i32;

        let result = match (s).checked_sub(t) {
            Some(value) => value as u32,
            None => return Err(CpuException::ArithmeticOverflow),
        };

        self.regs.set_r(rd, result);

        Ok(())
    }

    /// Sub Unsigned
    fn subu(&mut self, instruction: Instruction) {
        let rs = instruction.rs();
        let rt = instruction.rt();
        let rd = instruction.rd();

        let result = self.regs.get_r(rs).wrapping_sub(self.regs.get_r(rt));

        self.regs.set_r(rd, result);
    }

    /// Bitwise And
    fn and(&mut self, instruction: Instruction) {
        let rs = instruction.rs();
        let rt = instruction.rt();
        let rd = instruction.rd();

        let result = self.regs.get_r(rs) & self.regs.get_r(rt);

        self.regs.set_r(rd, result);
    }

    /// Bitwise Or
    fn or(&mut self, instruction: Instruction) {
        let rs = instruction.rs();
        let rt = instruction.rt();
        let rd = instruction.rd();

        let value = self.regs.get_r(rs) | self.regs.get_r(rt);

        self.regs.set_r(rd, value);
    }

    fn xor(&mut self, instruction: Instruction) {
        let rs = instruction.rs();
        let rt = instruction.rt();
        let rd = instruction.rd();

        let value = self.regs.get_r(rs) ^ self.regs.get_r(rt);

        self.regs.set_r(rd, value);
    }

    fn nor(&mut self, instruction: Instruction) {
        let rs = instruction.rs();
        let rt = instruction.rt();
        let rd = instruction.rd();

        let value = !(self.regs.get_r(rs) | self.regs.get_r(rt));

        self.regs.set_r(rd, value);
    }

    /// Set on Less Than
    fn slt(&mut self, instruction: Instruction) {
        let rd = instruction.rd();
        let rs = instruction.rs();
        let rt = instruction.rt();

        let result = (self.regs.get_r(rs) as i32) < (self.regs.get_r(rt) as i32);

        self.regs.set_r(rd, result as u32);
    }

    /// Set on Less Than Unsigned
    fn sltu(&mut self, instruction: Instruction) {
        let rd = instruction.rd();
        let rs = instruction.rs();
        let rt = instruction.rt();

        let result = self.regs.get_r(rs) < self.regs.get_r(rt);

        self.regs.set_r(rd, result as u32);
    }
}

impl<Mem: MemoryInterface> Cpu<Mem> {
    // BcondZ

    /// Branch if Less Than Zero
    fn bltz(&mut self, instruction: Instruction) {
        let offset = instruction.imm_sign_extended();
        let rs = instruction.rs();

        let s = self.regs.get_r(rs) as i32;

        let take = s < 0;

        self.branch(offset, take);
    }

    /// Branch if Greater than or Equal to Zero
    fn bgez(&mut self, instruction: Instruction) {
        let offset = instruction.imm_sign_extended();
        let rs = instruction.rs();

        let s = self.regs.get_r(rs) as i32;

        let take = s >= 0;

        self.branch(offset, take);
    }

    /// Branch if Less Than Zero And Link
    fn bltzal(&mut self, instruction: Instruction) {
        let offset = instruction.imm_sign_extended();
        let rs = instruction.rs();

        let s = self.regs.get_r(rs) as i32;

        let take = s < 0;

        self.branch(offset, take);

        self.regs
            .set_r(RegisterIndex(31), self.regs.pc.wrapping_add(4));
    }

    /// Branch if Greater than or Equal to Zero And Link
    fn bgezal(&mut self, instruction: Instruction) {
        let offset = instruction.imm_sign_extended();
        let rs = instruction.rs();

        let s = self.regs.get_r(rs) as i32;

        let take = s >= 0;

        self.branch(offset, take);

        self.regs
            .set_r(RegisterIndex(31), self.regs.pc.wrapping_add(4));
    }
}

impl<Mem: MemoryInterface> Cpu<Mem> {
    // COP

    /// Move From Coprocessor 0
    fn mfc0(&mut self, instruction: Instruction) {
        let cpu_r = instruction.rt();
        let cop_r = instruction.rd().0;

        let value = match cop_r {
            12 => self.regs.cop0.sr.read(),
            13 => self.regs.cop0.cause.read(),
            14 => self.regs.cop0.epc,

            _ => {
                todo!("Unhandled read from COP0");
            }
        };

        self.regs.delayed_load = Some((cpu_r, value));
    }

    fn mtc0(&mut self, instruction: Instruction) {
        let cpu_r = instruction.rt();
        let cop_r = instruction.rd().0;

        let value = self.regs.get_r(cpu_r);

        match cop_r {
            3 | 5 | 6 | 7 | 9 | 11 => {
                // Breakpoint registers
                assert!(value == 0, "unhandled write to COP0");
            }

            12 => self.regs.cop0.sr.write(value),

            13 => {
                // CAUSE
                assert!(value == 0, "unhandled write to COP0");
            }

            nn => unimplemented!("Unimplemented COP0 register: {nn}"),
        }
    }

    fn rfe(&mut self, instruction: Instruction) {
        if instruction.0 & 0x3F != 0b010000 {
            panic!("Invalid cop0 instruction: {instruction}")
        }

        let mode = self.regs.cop0.sr.raw & 0x3F;
        self.regs.cop0.sr.raw &= !0x3F;
        self.regs.cop0.sr.raw |= mode >> 2;
    }
}

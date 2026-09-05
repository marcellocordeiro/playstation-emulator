use strum::Display;

use crate::components::cpu::instruction::{Instruction, RegisterIndex};

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq)]
pub enum DecodedInstruction {
    #[strum(to_string = "sll {rd}, {rt}, {shamt}")]
    Sll {
        rd: RegisterIndex,
        rt: RegisterIndex,
        shamt: u32,
    },

    #[strum(to_string = "srl {rd}, {rt}, {shamt}")]
    Srl {
        rd: RegisterIndex,
        rt: RegisterIndex,
        shamt: u32,
    },

    #[strum(to_string = "sra {rd}, {rt}, {shamt}")]
    Sra {
        rd: RegisterIndex,
        rt: RegisterIndex,
        shamt: u32,
    },

    #[strum(to_string = "sllv {rd}, {rt}, {rs}")]
    Sllv {
        rd: RegisterIndex,
        rt: RegisterIndex,
        rs: RegisterIndex,
    },

    #[strum(to_string = "srlv {rd}, {rt}, {rs}")]
    Srlv {
        rd: RegisterIndex,
        rt: RegisterIndex,
        rs: RegisterIndex,
    },

    #[strum(to_string = "srav {rd}, {rt}, {rs}")]
    Srav {
        rd: RegisterIndex,
        rt: RegisterIndex,
        rs: RegisterIndex,
    },

    #[strum(to_string = "jr {rs}")]
    Jr { rs: RegisterIndex },

    #[strum(to_string = "jalr {rd}, {rs}")]
    Jalr {
        rd: RegisterIndex,
        rs: RegisterIndex,
    },

    #[strum(to_string = "syscall")]
    Syscall,

    #[strum(to_string = "break")]
    Break,

    #[strum(to_string = "mfhi {rd}")]
    Mfhi { rd: RegisterIndex },

    #[strum(to_string = "mthi {rs}")]
    Mthi { rs: RegisterIndex },

    #[strum(to_string = "mflo {rd}")]
    Mflo { rd: RegisterIndex },

    #[strum(to_string = "mtlo {rs}")]
    Mtlo { rs: RegisterIndex },

    #[strum(to_string = "mult {rs}, {rt}")]
    Mult {
        rs: RegisterIndex,
        rt: RegisterIndex,
    },

    #[strum(to_string = "multu {rs}, {rt}")]
    Multu {
        rs: RegisterIndex,
        rt: RegisterIndex,
    },

    #[strum(to_string = "div {rs}, {rt}")]
    Div {
        rs: RegisterIndex,
        rt: RegisterIndex,
    },

    #[strum(to_string = "divu {rs}, {rt}")]
    Divu {
        rs: RegisterIndex,
        rt: RegisterIndex,
    },

    #[strum(to_string = "add {rd}, {rs}, {rt}")]
    Add {
        rd: RegisterIndex,
        rs: RegisterIndex,
        rt: RegisterIndex,
    },

    #[strum(to_string = "addu {rd}, {rs}, {rt}")]
    Addu {
        rd: RegisterIndex,
        rs: RegisterIndex,
        rt: RegisterIndex,
    },

    #[strum(to_string = "sub {rd}, {rs}, {rt}")]
    Sub {
        rd: RegisterIndex,
        rs: RegisterIndex,
        rt: RegisterIndex,
    },

    #[strum(to_string = "subu {rd}, {rs}, {rt}")]
    Subu {
        rd: RegisterIndex,
        rs: RegisterIndex,
        rt: RegisterIndex,
    },

    #[strum(to_string = "and {rd}, {rs}, {rt}")]
    And {
        rd: RegisterIndex,
        rs: RegisterIndex,
        rt: RegisterIndex,
    },

    #[strum(to_string = "or {rd}, {rs}, {rt}")]
    Or {
        rd: RegisterIndex,
        rs: RegisterIndex,
        rt: RegisterIndex,
    },

    #[strum(to_string = "xor {rd}, {rs}, {rt}")]
    Xor {
        rd: RegisterIndex,
        rs: RegisterIndex,
        rt: RegisterIndex,
    },

    #[strum(to_string = "nor {rd}, {rs}, {rt}")]
    Nor {
        rd: RegisterIndex,
        rs: RegisterIndex,
        rt: RegisterIndex,
    },

    #[strum(to_string = "slt {rd}, {rs}, {rt}")]
    Slt {
        rd: RegisterIndex,
        rs: RegisterIndex,
        rt: RegisterIndex,
    },

    #[strum(to_string = "sltu {rd}, {rs}, {rt}")]
    Sltu {
        rd: RegisterIndex,
        rs: RegisterIndex,
        rt: RegisterIndex,
    },

    #[strum(to_string = "bltz {rs}, {offset}")]
    Bltz { rs: RegisterIndex, offset: u32 },

    #[strum(to_string = "bgez {rs}, {offset}")]
    Bgez { rs: RegisterIndex, offset: u32 },

    #[strum(to_string = "bltzal {rs}, {offset}")]
    Bltzal { rs: RegisterIndex, offset: u32 },

    #[strum(to_string = "bgezal {rs}, {offset}")]
    Bgezal { rs: RegisterIndex, offset: u32 },

    #[strum(to_string = "j ${target:#010x}")]
    J { target: u32 },

    #[strum(to_string = "jal ${target:#010x}")]
    Jal { target: u32 },

    #[strum(to_string = "beq {rs}, {rt}, {offset}")]
    Beq {
        rs: RegisterIndex,
        rt: RegisterIndex,
        offset: u32,
    },

    #[strum(to_string = "bne {rs}, {rt}, {offset}")]
    Bne {
        rs: RegisterIndex,
        rt: RegisterIndex,
        offset: u32,
    },

    #[strum(to_string = "blez {rs}, {offset}")]
    Blez { rs: RegisterIndex, offset: u32 },

    #[strum(to_string = "bgtz {rs}, {offset}")]
    Bgtz { rs: RegisterIndex, offset: u32 },

    #[strum(to_string = "addi {rt}, {rs}, {imm_se:#06x}")]
    Addi {
        rt: RegisterIndex,
        rs: RegisterIndex,
        imm_se: u32,
    },

    #[strum(to_string = "addiu {rt}, {rs}, {imm_se:#06x}")]
    Addiu {
        rt: RegisterIndex,
        rs: RegisterIndex,
        imm_se: u32,
    },

    #[strum(to_string = "slti {rt}, {rs}, {imm_se:#06x}")]
    Slti {
        rt: RegisterIndex,
        rs: RegisterIndex,
        imm_se: u32,
    },

    #[strum(to_string = "sltiu {rt}, {rs}, {imm_se:#06x}")]
    Sltiu {
        rt: RegisterIndex,
        rs: RegisterIndex,
        imm_se: u32,
    },

    #[strum(to_string = "andi {rt}, {rs}, {imm:#06x}")]
    Andi {
        rt: RegisterIndex,
        rs: RegisterIndex,
        imm: u32,
    },

    #[strum(to_string = "ori {rt}, {rs}, {imm:#06x}")]
    Ori {
        rt: RegisterIndex,
        rs: RegisterIndex,
        imm: u32,
    },

    #[strum(to_string = "xori {rt}, {rs} {imm:#06x}")]
    Xori {
        rt: RegisterIndex,
        rs: RegisterIndex,
        imm: u32,
    },

    #[strum(to_string = "lui {rt}, {imm:#06x}")]
    Lui { rt: RegisterIndex, imm: u32 },

    #[strum(to_string = "cop0")]
    Cop0,

    #[strum(to_string = "cop1")]
    Cop1,

    #[strum(to_string = "cop2")]
    Cop2,

    #[strum(to_string = "cop3")]
    Cop3,

    #[strum(to_string = "lb {rt}, {offset:#06x}({base})")]
    Lb {
        rt: RegisterIndex,
        offset: u32,
        base: RegisterIndex,
    },

    #[strum(to_string = "lh {rt}, {offset:#06x}({base})")]
    Lh {
        rt: RegisterIndex,
        offset: u32,
        base: RegisterIndex,
    },

    #[strum(to_string = "lwl {rt}, {offset:#06x}({base})")]
    Lwl {
        rt: RegisterIndex,
        offset: u32,
        base: RegisterIndex,
    },

    #[strum(to_string = "lw {rt}, {offset:#06x}({base})")]
    Lw {
        rt: RegisterIndex,
        offset: u32,
        base: RegisterIndex,
    },

    #[strum(to_string = "lbu {rt}, {offset:#06x}({base})")]
    Lbu {
        rt: RegisterIndex,
        offset: u32,
        base: RegisterIndex,
    },

    #[strum(to_string = "lhu {rt}, {offset:#06x}({base})")]
    Lhu {
        rt: RegisterIndex,
        offset: u32,
        base: RegisterIndex,
    },

    #[strum(to_string = "lwr {rt}, {offset:#06x}({base})")]
    Lwr {
        rt: RegisterIndex,
        offset: u32,
        base: RegisterIndex,
    },

    #[strum(to_string = "sb {rt}, {offset:#06x}({base})")]
    Sb {
        rt: RegisterIndex,
        offset: u32,
        base: RegisterIndex,
    },

    #[strum(to_string = "sh {rt}, {offset:#06x}({base})")]
    Sh {
        rt: RegisterIndex,
        offset: u32,
        base: RegisterIndex,
    },

    #[strum(to_string = "swl {rt}, {offset:#06x}({base})")]
    Swl {
        rt: RegisterIndex,
        offset: u32,
        base: RegisterIndex,
    },

    #[strum(to_string = "sw {rt}, {offset:#06x}({base})")]
    Sw {
        rt: RegisterIndex,
        offset: u32,
        base: RegisterIndex,
    },

    #[strum(to_string = "swr {rt}, {offset:#06x}({base})")]
    Swr {
        rt: RegisterIndex,
        offset: u32,
        base: RegisterIndex,
    },

    #[strum(to_string = "lwc0")]
    Lwc0,

    #[strum(to_string = "lwc1")]
    Lwc1,

    #[strum(to_string = "lwc2")]
    Lwc2,

    #[strum(to_string = "lwc3")]
    Lwc3,

    #[strum(to_string = "swc0")]
    Swc0,

    #[strum(to_string = "swc1")]
    Swc1,

    #[strum(to_string = "swc2")]
    Swc2,

    #[strum(to_string = "swc3")]
    Swc3,

    #[strum(to_string = "UNKNOWN ({opcode:#010X})")]
    Invalid { opcode: u32 },
}

impl DecodedInstruction {
    #[must_use]
    pub fn decode(instruction: Instruction) -> Self {
        match instruction.primary() {
            0x00 => {
                match instruction.secondary() {
                    0x00 => {
                        Self::Sll {
                            rd: instruction.rd(),
                            rt: instruction.rt(),
                            shamt: instruction.shift_imm(),
                        }
                    }
                    0x02 => {
                        Self::Srl {
                            rd: instruction.rd(),
                            rt: instruction.rt(),
                            shamt: instruction.shift_imm(),
                        }
                    }
                    0x03 => {
                        Self::Sra {
                            rd: instruction.rd(),
                            rt: instruction.rt(),
                            shamt: instruction.shift_imm(),
                        }
                    }
                    0x04 => {
                        Self::Sllv {
                            rd: instruction.rd(),
                            rt: instruction.rt(),
                            rs: instruction.rs(),
                        }
                    }
                    0x06 => {
                        Self::Srlv {
                            rd: instruction.rd(),
                            rt: instruction.rt(),
                            rs: instruction.rs(),
                        }
                    }
                    0x07 => {
                        Self::Srav {
                            rd: instruction.rd(),
                            rt: instruction.rt(),
                            rs: instruction.rs(),
                        }
                    }
                    0x08 => {
                        Self::Jr {
                            rs: instruction.rs(),
                        }
                    }
                    0x09 => {
                        Self::Jalr {
                            rd: instruction.rd(),
                            rs: instruction.rs(),
                        }
                    }
                    0x0C => Self::Syscall,
                    0x0D => Self::Break,
                    0x10 => {
                        Self::Mfhi {
                            rd: instruction.rd(),
                        }
                    }
                    0x11 => {
                        Self::Mthi {
                            rs: instruction.rs(),
                        }
                    }
                    0x12 => {
                        Self::Mflo {
                            rd: instruction.rd(),
                        }
                    }
                    0x13 => {
                        Self::Mtlo {
                            rs: instruction.rs(),
                        }
                    }
                    0x18 => {
                        Self::Mult {
                            rs: instruction.rs(),
                            rt: instruction.rt(),
                        }
                    }
                    0x19 => {
                        Self::Multu {
                            rs: instruction.rs(),
                            rt: instruction.rt(),
                        }
                    }
                    0x1A => {
                        Self::Div {
                            rs: instruction.rs(),
                            rt: instruction.rt(),
                        }
                    }
                    0x1B => {
                        Self::Divu {
                            rs: instruction.rs(),
                            rt: instruction.rt(),
                        }
                    }
                    0x20 => {
                        Self::Add {
                            rd: instruction.rd(),
                            rs: instruction.rs(),
                            rt: instruction.rt(),
                        }
                    }
                    0x21 => {
                        Self::Addu {
                            rd: instruction.rd(),
                            rs: instruction.rs(),
                            rt: instruction.rt(),
                        }
                    }
                    0x22 => {
                        Self::Sub {
                            rd: instruction.rd(),
                            rs: instruction.rs(),
                            rt: instruction.rt(),
                        }
                    }
                    0x23 => {
                        Self::Subu {
                            rd: instruction.rd(),
                            rs: instruction.rs(),
                            rt: instruction.rt(),
                        }
                    }
                    0x24 => {
                        Self::And {
                            rd: instruction.rd(),
                            rs: instruction.rs(),
                            rt: instruction.rt(),
                        }
                    }
                    0x25 => {
                        Self::Or {
                            rd: instruction.rd(),
                            rs: instruction.rs(),
                            rt: instruction.rt(),
                        }
                    }
                    0x26 => {
                        Self::Xor {
                            rd: instruction.rd(),
                            rs: instruction.rs(),
                            rt: instruction.rt(),
                        }
                    }
                    0x27 => {
                        Self::Nor {
                            rd: instruction.rd(),
                            rs: instruction.rs(),
                            rt: instruction.rt(),
                        }
                    }
                    0x2A => {
                        Self::Slt {
                            rd: instruction.rd(),
                            rs: instruction.rs(),
                            rt: instruction.rt(),
                        }
                    }
                    0x2B => {
                        Self::Sltu {
                            rd: instruction.rd(),
                            rs: instruction.rs(),
                            rt: instruction.rt(),
                        }
                    }

                    _ => {
                        Self::Invalid {
                            opcode: instruction.0,
                        }
                    }
                }
            }

            0x01 => {
                match (instruction.0 >> 16) & 0x1F {
                    0x00 => {
                        Self::Bltz {
                            rs: instruction.rs(),
                            offset: instruction.branch_offset(),
                        }
                    }
                    0x01 => {
                        Self::Bgez {
                            rs: instruction.rs(),
                            offset: instruction.branch_offset(),
                        }
                    }
                    0x10 => {
                        Self::Bltzal {
                            rs: instruction.rs(),
                            offset: instruction.branch_offset(),
                        }
                    }
                    0x11 => {
                        Self::Bgezal {
                            rs: instruction.rs(),
                            offset: instruction.branch_offset(),
                        }
                    }
                    _ => {
                        Self::Invalid {
                            opcode: instruction.0,
                        }
                    }
                }
            }
            0x02 => {
                Self::J {
                    target: instruction.jump_target(),
                }
            }

            0x03 => {
                Self::Jal {
                    target: instruction.jump_target(),
                }
            }
            0x04 => {
                Self::Beq {
                    rs: instruction.rs(),
                    rt: instruction.rt(),
                    offset: instruction.branch_offset(),
                }
            }
            0x05 => {
                Self::Bne {
                    rs: instruction.rs(),
                    rt: instruction.rt(),
                    offset: instruction.branch_offset(),
                }
            }
            0x06 => {
                Self::Blez {
                    rs: instruction.rs(),
                    offset: instruction.branch_offset(),
                }
            }
            0x07 => {
                Self::Bgtz {
                    rs: instruction.rs(),
                    offset: instruction.branch_offset(),
                }
            }
            0x08 => {
                Self::Addi {
                    rt: instruction.rt(),
                    rs: instruction.rs(),
                    imm_se: instruction.imm_sign_extended(),
                }
            }
            0x09 => {
                Self::Addiu {
                    rt: instruction.rt(),
                    rs: instruction.rs(),
                    imm_se: instruction.imm_sign_extended(),
                }
            }
            0x0A => {
                Self::Slti {
                    rt: instruction.rt(),
                    rs: instruction.rs(),
                    imm_se: instruction.imm_sign_extended(),
                }
            }
            0x0B => {
                Self::Sltiu {
                    rt: instruction.rt(),
                    rs: instruction.rs(),
                    imm_se: instruction.imm_sign_extended(),
                }
            }
            0x0C => {
                Self::Andi {
                    rt: instruction.rt(),
                    rs: instruction.rs(),
                    imm: instruction.imm(),
                }
            }
            0x0D => {
                Self::Ori {
                    rt: instruction.rt(),
                    rs: instruction.rs(),
                    imm: instruction.imm(),
                }
            }
            0x0E => {
                Self::Xori {
                    rt: instruction.rt(),
                    rs: instruction.rs(),
                    imm: instruction.imm(),
                }
            }
            0x0F => {
                Self::Lui {
                    rt: instruction.rt(),
                    imm: instruction.imm(),
                }
            }
            0x10 => Self::Cop0,
            0x11 => Self::Cop1,
            0x12 => Self::Cop2,
            0x13 => Self::Cop3,
            0x20 => {
                Self::Lb {
                    rt: instruction.rt(),
                    offset: instruction.imm_sign_extended(),
                    base: instruction.rs(),
                }
            }
            0x21 => {
                Self::Lh {
                    rt: instruction.rt(),
                    offset: instruction.imm_sign_extended(),
                    base: instruction.rs(),
                }
            }
            0x22 => {
                Self::Lwl {
                    rt: instruction.rt(),
                    offset: instruction.imm_sign_extended(),
                    base: instruction.rs(),
                }
            }
            0x23 => {
                Self::Lw {
                    rt: instruction.rt(),
                    offset: instruction.imm_sign_extended(),
                    base: instruction.rs(),
                }
            }
            0x24 => {
                Self::Lbu {
                    rt: instruction.rt(),
                    offset: instruction.imm_sign_extended(),
                    base: instruction.rs(),
                }
            }
            0x25 => {
                Self::Lhu {
                    rt: instruction.rt(),
                    offset: instruction.imm_sign_extended(),
                    base: instruction.rs(),
                }
            }
            0x26 => {
                Self::Lwr {
                    rt: instruction.rt(),
                    offset: instruction.imm_sign_extended(),
                    base: instruction.rs(),
                }
            }
            0x28 => {
                Self::Sb {
                    rt: instruction.rt(),
                    offset: instruction.imm_sign_extended(),
                    base: instruction.rs(),
                }
            }
            0x29 => {
                Self::Sh {
                    rt: instruction.rt(),
                    offset: instruction.imm_sign_extended(),
                    base: instruction.rs(),
                }
            }
            0x2A => {
                Self::Swl {
                    rt: instruction.rt(),
                    offset: instruction.imm_sign_extended(),
                    base: instruction.rs(),
                }
            }
            0x2B => {
                Self::Sw {
                    rt: instruction.rt(),
                    offset: instruction.imm_sign_extended(),
                    base: instruction.rs(),
                }
            }
            0x2E => {
                Self::Swr {
                    rt: instruction.rt(),
                    offset: instruction.imm_sign_extended(),
                    base: instruction.rs(),
                }
            }
            0x30 => Self::Lwc0,
            0x31 => Self::Lwc1,
            0x32 => Self::Lwc2,
            0x33 => Self::Lwc3,
            0x38 => Self::Swc0,
            0x39 => Self::Swc1,
            0x3A => Self::Swc2,
            0x3B => Self::Swc3,

            _ => {
                Self::Invalid {
                    opcode: instruction.0,
                }
            }
        }
    }
}

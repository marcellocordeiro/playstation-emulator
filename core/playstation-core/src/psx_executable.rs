use crate::components::cpu::registers::Registers;

pub struct PsxExecutable;

impl PsxExecutable {
    pub fn apply(exe: &[u8], regs: &mut Registers, ram: &mut [u8]) {
        // Parse EXE header
        let initial_pc = u32::from_le_bytes(exe[0x10..0x14].try_into().unwrap());
        let initial_r28 = u32::from_le_bytes(exe[0x14..0x18].try_into().unwrap());
        let exe_ram_address = u32::from_le_bytes(exe[0x18..0x1C].try_into().unwrap()) & 0x001F_FFFF;
        let exe_size = u32::from_le_bytes(exe[0x1C..0x20].try_into().unwrap());
        let initial_sp = u32::from_le_bytes(exe[0x30..0x34].try_into().unwrap());

        let ram_slice = &mut ram[exe_ram_address as usize..(exe_ram_address + exe_size) as usize];
        let exe_slice = &exe[2048..2048 + exe_size as usize];

        // Copy EXE code/data into the PS1 RAM
        ram_slice.copy_from_slice(exe_slice);

        // Set initial register values
        regs.r[28] = initial_r28;
        if initial_sp != 0 {
            regs.r[29] = initial_sp;
            regs.r[30] = initial_sp;
        }

        // Jump to the EXE entry point; execution can continue normally after this
        regs.pc = initial_pc;
    }
}

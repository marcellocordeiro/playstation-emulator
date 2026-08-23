use crate::components::{
    cpu::Cpu,
    memory::{
        Memory,
        bios::{Bios, BiosData},
    },
};

pub struct PlayStation {
    pub cpu: Cpu,
}

impl PlayStation {
    #[must_use]
    pub fn new(bios_data: BiosData) -> Self {
        let bios = Bios::new(bios_data).unwrap();
        let memory = Memory::new(bios);

        let cpu = Cpu::new(memory);

        Self { cpu }
    }

    pub fn reset(&mut self) {}

    pub fn step(&mut self) {
        self.cpu.run_next_instruction();
    }

    pub fn run_frame(&mut self) {}
}

mod components;
pub mod constants;

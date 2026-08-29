use crate::components::{
    cpu::tests::single_step_tests::test_ram::TestRam,
    memory::{MemoryInterface, addressable::Addressable, bios::Bios},
};

pub struct TestMemory {
    ram: TestRam,
}

impl MemoryInterface for TestMemory {
    fn new(_bios: Bios) -> Self {
        Self {
            ram: TestRam::default(),
        }
    }

    fn load<T: Addressable>(&self, address: u32) -> T {
        self.ram.load(address)
    }

    fn store<T: Addressable>(&mut self, address: u32, value: T) {
        self.ram.store(address, value);
    }
}

impl TestMemory {}

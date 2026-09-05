use std::cell::RefCell;

use sst_r3000::{Cycle, CycleAction};

use crate::components::{
    cpu::tests::single_step_tests::test_ram::TestRam,
    memory::{MemoryInterface, addressable::Addressable, bios::Bios},
};

#[derive(Default)]
pub struct TestMemory {
    ram: TestRam,

    pub cycles: RefCell<Vec<Cycle>>,
}

impl MemoryInterface for TestMemory {
    fn new(_bios: Bios) -> Self {
        Self::default()
    }

    fn load<T: Addressable>(&self, address: u32) -> T {
        if address % (T::width() as u32) != 0 {
            panic!("Unaligned access not supported");
        }

        self.ram.load(address)
    }

    fn store<T: Addressable>(&mut self, address: u32, value: T) {
        if address % (T::width() as u32) != 0 {
            panic!("Unaligned access not supported");
        }

        self.ram.store(address, value);
    }

    // Loads

    fn fetch_instruction(&self, address: u32) -> u32 {
        let value = self.load(address);

        self.push_cycle(address, value, CycleAction::Fetch);

        value
    }

    fn load_byte(&self, address: u32) -> u8 {
        let value = self.load(address);

        self.push_cycle(address, value, CycleAction::Read);

        value
    }

    fn load_halfword(&self, address: u32) -> u16 {
        let value = self.load(address);

        self.push_cycle(address, value, CycleAction::Read);

        value
    }

    fn load_word(&self, address: u32) -> u32 {
        let value = self.load(address);

        self.push_cycle(address, value, CycleAction::Read);

        value
    }

    // Stores

    fn store_byte(&mut self, address: u32, value: u8) {
        self.store(address, value);

        self.push_cycle(address, value, CycleAction::Write);
    }

    fn store_halfword(&mut self, address: u32, value: u16) {
        self.store(address, value);

        self.push_cycle(address, value, CycleAction::Write);
    }

    fn store_word(&mut self, address: u32, value: u32) {
        self.store(address, value);

        self.push_cycle(address, value, CycleAction::Write);
    }

    fn ram(&self) -> &[u8] {
        unimplemented!();
    }

    fn ram_mut(&mut self) -> &mut [u8] {
        unimplemented!();
    }
}

impl TestMemory {
    pub fn from_opcode_and_cycles(opcode_addr: u32, opcode: u32, cycles: &[Cycle]) -> Self {
        let mut ram = TestRam::default();

        ram.store_word(opcode_addr, opcode);

        for cycle in cycles {
            let address = cycle.addr as u32;
            let value = cycle.val as u32;

            if address == opcode_addr {
                assert_eq!(value, opcode);
                continue;
            }

            match cycle.sz {
                1 => ram.store_byte(address, value as u8),
                2 => ram.store_halfword(address, value as u16),
                4 => ram.store_word(address, value),
                _ => panic!("invalid size number"),
            }

            ram.store_word(address, value);
        }

        Self {
            ram,
            ..Default::default()
        }
    }

    /// TODO
    fn push_cycle<T: Addressable>(&self, address: u32, value: T, action: CycleAction) {
        let actions = action;
        let sz = T::width() as u32;
        let addr = address as i64;
        let val = value.as_u32() as i64;

        self.cycles.borrow_mut().push(Cycle {
            actions,
            sz,
            addr,
            val,
        });
    }
}

use std::cell::RefCell;

use sst_r3000::Cycle;

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
        let value = self.ram.load(address);

        self.push_read_cycle::<T>(address);

        value
    }

    fn store<T: Addressable>(&mut self, address: u32, value: T) {
        self.push_cycle(address, value);

        self.ram.store(address, value);
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

        Self { ram, ..Default::default() }
    }

    fn push_read_cycle<T: Addressable>(&self, address: u32) {
        let aligned_value: u32 = self.ram.load(address & !0b11);

        let size = T::width() as u32;

        // let actions = size;
        let sz = size;
        let addr = address as i64;
        let val = aligned_value as i64;

        self.cycles.borrow_mut().push(Cycle {
            // actions,
            sz,
            addr,
            val,
        });
    }

    fn push_cycle<T: Addressable>(&self, address: u32, value: T) {
        let size = T::width() as u32;

        // let actions = size;
        let sz = size;
        let addr = address as i64;
        let val = value.as_u32() as i64;

        self.cycles.borrow_mut().push(Cycle {
            // actions,
            sz,
            addr,
            val,
        });
    }
}

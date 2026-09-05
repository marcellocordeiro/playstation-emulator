use crate::components::memory::addressable::{AccessWidth, Addressable};

pub const SCRATCHPAD_SIZE: usize = 1024;

pub struct Scratchpad {
    pub data: Box<[u8; SCRATCHPAD_SIZE]>,
}

impl Scratchpad {
    #[must_use]
    pub fn new() -> Self {
        let data = vec![0; SCRATCHPAD_SIZE].try_into().unwrap();

        Self { data }
    }

    #[must_use]
    pub fn load<T: Addressable>(&self, offset: u32) -> T {
        match T::width() {
            AccessWidth::Byte => T::from_u32(self.load_byte(offset) as u32),
            AccessWidth::Halfword => T::from_u32(self.load_halfword(offset) as u32),
            AccessWidth::Word => T::from_u32(self.load_word(offset)),
        }
    }

    pub fn store<T: Addressable>(&mut self, offset: u32, value: T) {
        match T::width() {
            AccessWidth::Byte => self.store_byte(offset, value.as_u8()),
            AccessWidth::Halfword => self.store_halfword(offset, value.as_u16()),
            AccessWidth::Word => self.store_word(offset, value.as_u32()),
        }
    }

    #[must_use]
    pub fn load_word(&self, offset: u32) -> u32 {
        let offset = offset as usize;

        u32::from_le_bytes([
            self.data[offset],
            self.data[offset + 1],
            self.data[offset + 2],
            self.data[offset + 3],
        ])
    }

    #[must_use]
    pub fn load_halfword(&self, offset: u32) -> u16 {
        let offset = offset as usize;

        u16::from_le_bytes([self.data[offset], self.data[offset + 1]])
    }

    #[must_use]
    pub fn load_byte(&self, offset: u32) -> u8 {
        let offset = offset as usize;

        self.data[offset]
    }

    pub fn store_word(&mut self, offset: u32, value: u32) {
        let offset = offset as usize;

        self.data[offset..(offset + 4)].copy_from_slice(&value.to_le_bytes());
    }

    pub fn store_halfword(&mut self, offset: u32, value: u16) {
        let offset = offset as usize;

        self.data[offset..(offset + 2)].copy_from_slice(&value.to_le_bytes());
    }

    pub fn store_byte(&mut self, offset: u32, value: u8) {
        let offset = offset as usize;

        self.data[offset] = value;
    }
}

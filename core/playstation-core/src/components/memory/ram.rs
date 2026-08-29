use crate::components::memory::addressable::{AccessWidth, Addressable};

pub struct Ram {
    data: Vec<u8>,
}

impl Ram {
    pub fn new() -> Self {
        let data = vec![0; 2 * 1024 * 1024];

        Self { data }
    }

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

    #[expect(clippy::identity_op)]
    pub fn load_word(&self, offset: u32) -> u32 {
        let b0 = self.load_byte(offset + 0) as u32;
        let b1 = self.load_byte(offset + 1) as u32;
        let b2 = self.load_byte(offset + 2) as u32;
        let b3 = self.load_byte(offset + 3) as u32;

        b0 | (b1 << 8) | (b2 << 16) | (b3 << 24)
    }

    #[expect(clippy::identity_op)]
    pub fn load_halfword(&self, offset: u32) -> u16 {
        let b0 = self.load_byte(offset + 0) as u16;
        let b1 = self.load_byte(offset + 1) as u16;

        b0 | (b1 << 8)
    }

    pub fn load_byte(&self, offset: u32) -> u8 {
        let offset = offset as usize;

        self.data[offset]
    }

    #[expect(clippy::identity_op)]
    pub fn store_word(&mut self, offset: u32, value: u32) {
        let b0 = (value >> 0) as u8;
        let b1 = (value >> 8) as u8;
        let b2 = (value >> 16) as u8;
        let b3 = (value >> 24) as u8;

        self.store_byte(offset + 0, b0);
        self.store_byte(offset + 1, b1);
        self.store_byte(offset + 2, b2);
        self.store_byte(offset + 3, b3);
    }

    #[expect(clippy::identity_op)]
    pub fn store_halfword(&mut self, offset: u32, value: u16) {
        let b0 = (value >> 0) as u8;
        let b1 = (value >> 8) as u8;

        self.store_byte(offset + 0, b0);
        self.store_byte(offset + 1, b1);
    }

    pub fn store_byte(&mut self, offset: u32, value: u8) {
        let offset = offset as usize;

        self.data[offset] = value;
    }
}

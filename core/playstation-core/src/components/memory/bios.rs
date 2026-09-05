use md5::Digest;
use tracing::info;

use crate::components::memory::addressable::{AccessWidth, Addressable};

pub const BIOS_SIZE: usize = 512 * 1024; // 512K
pub type BiosData = [u8; BIOS_SIZE];

pub const BIOS_EXTENSIONS: [&str; 1] = ["bin"];
pub const BIOS_EXTENSIONS_DESCRIPTION: &str = "PlayStation BIOS";

pub struct Bios {
    data: Box<BiosData>,
    metadata: &'static Metadata,
}

impl Bios {
    pub fn new(data: Box<BiosData>) -> Result<Self, &'static str> {
        let hash = hex::encode(md5::Md5::digest(data.as_ref()));

        info!("BIOS md5: {hash}");

        let Some(metadata) = SUPPORTED_BIOS.iter().find(|m| m.md5 == hash) else {
            return Err("BIOS not supported");
        };

        Ok(Self { data, metadata })
    }

    #[must_use]
    pub fn new_dummy() -> Self {
        let data = vec![0_u8; BIOS_SIZE].into_boxed_slice().try_into().unwrap();
        Self {
            data,
            metadata: &SUPPORTED_BIOS[0],
        }
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

pub struct Metadata {
    name: &'static str,
    md5: &'static str,
}

impl Metadata {
    const fn new(name: &'static str, md5: &'static str) -> Self {
        Self { name, md5 }
    }
}

const SUPPORTED_BIOS: [Metadata; 1] = [Metadata::new(
    "scph1001",
    "924e392ed05558ffdb115408c263dccf",
)];

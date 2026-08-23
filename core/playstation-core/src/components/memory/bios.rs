use md5::Digest;
use tracing::info;

pub const BIOS_SIZE: usize = 512 * 1024; // 512K
pub type BiosData = [u8; BIOS_SIZE];

pub const BIOS_EXTENSIONS: [&str; 1] = ["bin"];
pub const BIOS_EXTENSIONS_DESCRIPTION: &str = "PlayStation BIOS";

pub struct Bios {
    data: BiosData,
    metadata: &'static Metadata,
}

impl Bios {
    pub fn new(data: BiosData) -> Result<Self, &'static str> {
        let hash = hex::encode(md5::Md5::digest(&data));

        info!("BIOS md5: {hash}");

        let Some(metadata) = SUPPORTED_BIOS.iter().find(|m| m.md5 == hash) else {
            return Err("BIOS not supported");
        };

        Ok(Self { data, metadata })
    }

    pub fn load_dword(&self, offset: u32) -> u32 {
        let offset = offset as usize;

        let load = |i: usize| self.data[i] as u32;

        let b0 = load(offset + 0);
        let b1 = load(offset + 1);
        let b2 = load(offset + 2);
        let b3 = load(offset + 3);

        b0 | (b1 << 8) | (b2 << 16) | (b3 << 24)
    }
}

pub struct Metadata {
    name: &'static str,
    md5: &'static str,
}

impl Metadata {
    const fn new(name: &'static str, md5: &'static str) -> Self {
        Metadata { name, md5 }
    }
}

const SUPPORTED_BIOS: [Metadata; 1] = [Metadata::new(
    "scph1001",
    "924e392ed05558ffdb115408c263dccf",
)];

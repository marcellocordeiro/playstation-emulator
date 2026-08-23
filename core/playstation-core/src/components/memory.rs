use tracing::info;

use crate::components::memory::bios::Bios;

pub struct Memory {
    bios: Bios,
}

impl Memory {
    pub fn new(bios: Bios) -> Self {
        Self { bios }
    }

    pub fn load_dword(&self, address: u32) -> u32 {
        use map::{Map, mapped_to};

        if address % 4 != 0 {
            panic!("Unaligned access not supported");
        }

        let Some(map) = mapped_to(address) else {
            panic!("Invalid or unimplemented map");
        };

        match map {
            Map::MemControl(offset) => {
                match offset {
                    _ => {
                        info!("Unhandled read to MEM_CONTROL");

                        0xFFFF_FFFF
                    }
                }
            }

            Map::RamSize(_) => {
                info!("Unhandled read to RAM_SIZE");

                0xFFFF_FFFF
            }

            Map::CacheControl(_) => {
                info!("Unhandled read to CACHE_CONTROL_RANGE");

                0xFFFF_FFFF
            }

            Map::Bios(offset) => self.bios.load_dword(offset),
        }
    }

    pub fn store_dword(&self, address: u32, value: u32) {
        use map::{Map, mapped_to};

        if address % 4 != 0 {
            panic!("Unaligned access not supported");
        }

        let Some(map) = mapped_to(address) else {
            panic!("Invalid or unimplemented map");
        };

        match map {
            Map::MemControl(offset) => {
                match offset {
                    0 => {
                        if value != 0x1F00_0000 {
                            panic!()
                        }
                    }

                    4 => {
                        if value != 0x1F80_2000 {
                            panic!()
                        }
                    }

                    _ => {
                        info!("Unhandled write to MEM_CONTROL");
                    }
                }
            }

            Map::RamSize(_) => {
                info!("Unhandled write to RAM_SIZE");
            }

            Map::CacheControl(_) => {
                info!("Unhandled write to CACHE_CONTROL_RANGE");
            }

            Map::Bios(_) => (), //self.bios.load_dword(offset),
        }
    }
}

/// | KUSEG       | KSEG0       | KSEG1       | Length | Description        |
/// | ----------- | ----------- | ----------- | ------ | ------------------ |
/// | 0x0000_0000 | 0x8000_0000 | 0xA000_0000 |  2048K | Main RAM           |
/// | 0x1F00_0000 | 0x9F00_0000 | 0xBF00_0000 |  8192K | Expansion Region 1 |
/// | 0x1F80_0000 | 0x9F80_0000 | 0xBF80_0000 |     1K | Scratchpad         |
/// | 0x1F80_1000 | 0x9F80_1000 | 0xBF80_1000 |     8K | Hardware registers |
/// | 0x1FC0_0000 | 0x9FC0_0000 | 0xBFC0_0000 |   512K | BIOS ROM           |
///
/// | KSEG2       | Length | Description |
/// | ----------- | ------ | ----------- |
/// | 0xFFFE_0000 |   512B | I/O Ports   |
mod map {
    use core::range::Range;

    const fn create_range_k(start: u32, length: u32) -> Range<u32> {
        Range {
            start,
            end: start + (length * 1024),
        }
    }

    const fn create_range(start: u32, length: u32) -> Range<u32> {
        Range {
            start,
            end: start + length,
        }
    }

    pub const MEM_CONTROL_RANGE: Range<u32> = create_range(0x1F801000, 36);

    pub const RAM_SIZE_RANGE: Range<u32> = create_range(0x1F801060, 4);

    pub const CACHE_CONTROL_RANGE: Range<u32> = create_range(0xFFFE0130, 4);

    pub const BIOS_RANGE_KUSEG: Range<u32> = create_range_k(0x1FC0_0000, 512);
    pub const BIOS_RANGE_KUSE0: Range<u32> = create_range_k(0x9FC0_0000, 512);
    pub const BIOS_RANGE_KUSE1: Range<u32> = create_range_k(0xBFC0_0000, 512);

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Map {
        MemControl(u32),
        RamSize(u32),
        CacheControl(u32),
        Bios(u32),
    }

    pub fn mapped_to(address: u32) -> Option<Map> {
        if MEM_CONTROL_RANGE.contains(&address) {
            return Some(Map::MemControl(address - MEM_CONTROL_RANGE.start));
        }

        if RAM_SIZE_RANGE.contains(&address) {
            return Some(Map::RamSize(address - RAM_SIZE_RANGE.start));
        }

        if CACHE_CONTROL_RANGE.contains(&address) {
            return Some(Map::CacheControl(address - CACHE_CONTROL_RANGE.start));
        }

        const BIOS: [Range<u32>; 3] = [BIOS_RANGE_KUSEG, BIOS_RANGE_KUSE0, BIOS_RANGE_KUSE1];

        for range in BIOS {
            if range.contains(&address) {
                return Some(Map::Bios(address - range.start));
            }
        }

        None
    }
}

pub mod bios;

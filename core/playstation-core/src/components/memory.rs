use tracing::info;

use crate::components::memory::{
    addressable::Addressable,
    bios::Bios,
    map::{Map, mapped_to, with_masked_region},
    ram::Ram,
    scratchpad::Scratchpad,
};

pub struct Memory {
    ram: Ram,
    scratchpad: Scratchpad,
    bios: Bios,
}

pub trait MemoryInterface {
    fn new(bios: Bios) -> Self;

    fn load<T: Addressable>(&self, address: u32) -> T;
    fn store<T: Addressable>(&mut self, address: u32, value: T);

    fn fetch_instruction(&self, address: u32) -> u32 {
        self.load(address)
    }

    fn load_word(&self, address: u32) -> u32 {
        self.load(address)
    }

    fn load_halfword(&self, address: u32) -> u16 {
        self.load(address)
    }

    fn load_byte(&self, address: u32) -> u8 {
        self.load(address)
    }

    fn store_word(&mut self, address: u32, value: u32) {
        self.store(address, value);
    }

    fn store_halfword(&mut self, address: u32, value: u16) {
        self.store(address, value);
    }

    fn store_byte(&mut self, address: u32, value: u8) {
        self.store(address, value);
    }

    fn ram(&self) -> &[u8];

    fn ram_mut(&mut self) -> &mut [u8];
}

impl MemoryInterface for Memory {
    fn new(bios: Bios) -> Self {
        Self {
            ram: Ram::new(),
            scratchpad: Scratchpad::new(),
            bios,
        }
    }

    fn load<T: Addressable>(&self, address: u32) -> T {
        if address % (T::width() as u32) != 0 {
            panic!("Unaligned access not supported");
        }

        let Some(map) = mapped_to(address) else {
            panic!(
                "Invalid or unimplemented map for {address:#08X} (resolved to: {resolved:#08X})",
                resolved = with_masked_region(address)
            );
        };

        match map {
            Map::SysControl(offset) => {
                match offset {
                    _ => {
                        // info!("Unhandled read to MEM_CONTROL");

                        T::stubbed()
                    }
                }
            }

            Map::RamSize(_) => {
                // info!("Unhandled read to RAM_SIZE");

                T::stubbed()
            }

            Map::Ram(offset) => self.ram.load(offset),

            Map::CacheControl(_) => {
                // info!("Unhandled read to CACHE_CONTROL_RANGE");

                T::stubbed()
            }

            Map::Bios(offset) => self.bios.load(offset),
            Map::Spu(_) => T::from_u32(0),
            Map::Expansion1(_) => T::from_u32(0),
            Map::Expansion2(_) => T::from_u32(0),

            Map::IrqControl(_) => T::stubbed(),
            Map::Timers(_) => T::stubbed(),
            Map::Dma(_) => T::from_u32(0),
            Map::Scratchpad(offset) => self.scratchpad.load(offset),
            Map::Gpu(_) => T::stubbed(),
        }
    }

    fn store<T: Addressable>(&mut self, address: u32, value: T) {
        if address % (T::width() as u32) != 0 {
            panic!("Unaligned access not supported");
        }

        let Some(map) = mapped_to(address) else {
            panic!(
                "Invalid or unimplemented map for {:08X}",
                with_masked_region(address)
            );
        };

        match map {
            Map::SysControl(offset) => {
                match offset {
                    0 => {
                        if value.as_u32() != 0x1F00_0000 {
                            panic!()
                        }
                    }

                    4 => {
                        if value.as_u32() != 0x1F80_2000 {
                            panic!()
                        }
                    }

                    _ => {
                        // info!("Unhandled write to MEM_CONTROL");
                    }
                }
            }

            Map::RamSize(_) => {
                // info!("Unhandled write to RAM_SIZE");
            }

            Map::Ram(offset) => self.ram.store(offset, value),

            Map::CacheControl(_) => {
                // info!("Unhandled write to CACHE_CONTROL_RANGE");
            }

            Map::Bios(_) => (),
            Map::Spu(_) => {}
            Map::Expansion1(_) => {}
            Map::Expansion2(_) => {}
            Map::IrqControl(_) => {}
            Map::Timers(_) => {}
            Map::Dma(_) => {}
            Map::Scratchpad(offset) => self.scratchpad.store(offset, value),
            Map::Gpu(_) => {}
        }
    }

    fn ram(&self) -> &[u8] {
        self.ram.data.as_slice()
    }

    fn ram_mut(&mut self) -> &mut [u8] {
        self.ram.data.as_mut_slice()
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
    type Range = core::range::Range<u32>;

    const fn create_range(start: u32, length: u32) -> Range {
        Range {
            start,
            end: start + length,
        }
    }

    pub fn with_masked_region(address: u32) -> u32 {
        let mask = match address >> 29 {
            0b000..=0b011 => 0xFFFF_FFFF, // KUSEG: 2048MB
            0b100 => 0x7FFF_FFFF,         // KSEG0: 512Mb
            0b101 => 0x1FFF_FFFF,         // KSEG1: 512MB
            0b110..=0b111 => 0xFFFF_FFFF, // KSEG2: 1024MB

            _ => unreachable!(),
        };

        address & mask
    }

    pub const RAM_RANGE: Range = create_range(0x0000_0000, 2 * 1024 * 1024);
    pub const BIOS_RANGE: Range = create_range(0x1FC0_0000, 512 * 1024);

    pub const SYS_CONTROL_RANGE: Range = create_range(0x1F80_1000, 36);

    pub const RAM_SIZE_RANGE: Range = create_range(0x1F80_1060, 4);

    pub const CACHE_CONTROL_RANGE: Range = create_range(0xFFFE_0130, 4);

    pub const SPU_RANGE: Range = create_range(0x1F80_1C00, 640);

    pub const EXPANSION_1_RANGE: Range = create_range(0x1F000000, 512 * 1024);
    pub const EXPANSION_2_RANGE: Range = create_range(0x1F80_2000, 66);

    pub const IRQ_CONTROL_RANGE: Range = create_range(0x1F80_1070, 8);

    pub const TIMERS_RANGE: Range = create_range(0x1F80_1100, 48); // ???

    pub const DMA_RANGE: Range = create_range(0x1F80_1080, 0x80);

    pub const SCRATCHPAD_RANGE: Range = create_range(0x1F80_0000, 0x400);

    pub const GPU_RANGE: Range = create_range(0x1F80_1810, 0x1F80_1818 - 0x1F80_1810 + 1);

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Map {
        Ram(u32),
        Scratchpad(u32),
        Bios(u32),
        SysControl(u32),
        RamSize(u32),
        CacheControl(u32),
        Spu(u32),
        Expansion1(u32),
        Expansion2(u32),
        IrqControl(u32),
        Timers(u32),
        Dma(u32),
        Gpu(u32),
    }

    pub fn mapped_to(address: u32) -> Option<Map> {
        let address = with_masked_region(address);

        if RAM_RANGE.contains(&address) {
            return Some(Map::Ram(address - RAM_RANGE.start));
        }

        if BIOS_RANGE.contains(&address) {
            return Some(Map::Bios(address - BIOS_RANGE.start));
        }

        if SYS_CONTROL_RANGE.contains(&address) {
            return Some(Map::SysControl(address - SYS_CONTROL_RANGE.start));
        }

        if RAM_SIZE_RANGE.contains(&address) {
            return Some(Map::RamSize(address - RAM_SIZE_RANGE.start));
        }

        if CACHE_CONTROL_RANGE.contains(&address) {
            return Some(Map::CacheControl(address - CACHE_CONTROL_RANGE.start));
        }

        if SPU_RANGE.contains(&address) {
            return Some(Map::Spu(address - SPU_RANGE.start));
        }

        if EXPANSION_1_RANGE.contains(&address) {
            return Some(Map::Expansion1(address - EXPANSION_1_RANGE.start));
        }

        if EXPANSION_2_RANGE.contains(&address) {
            return Some(Map::Expansion2(address - EXPANSION_2_RANGE.start));
        }

        if IRQ_CONTROL_RANGE.contains(&address) {
            return Some(Map::IrqControl(address - IRQ_CONTROL_RANGE.start));
        }

        if TIMERS_RANGE.contains(&address) {
            return Some(Map::Timers(address - TIMERS_RANGE.start));
        }

        if DMA_RANGE.contains(&address) {
            return Some(Map::Dma(address - DMA_RANGE.start));
        }

        if SCRATCHPAD_RANGE.contains(&address) {
            return Some(Map::Scratchpad(address - SCRATCHPAD_RANGE.start));
        }

        if GPU_RANGE.contains(&address) {
            return Some(Map::Gpu(address - GPU_RANGE.start));
        }

        None
    }
}

pub mod bios;
mod ram;
mod scratchpad;

pub mod addressable {
    use num::PrimInt;

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum AccessWidth {
        Byte = 1,
        Halfword = 2,
        Word = 4,
    }

    pub trait Addressable: PrimInt {
        fn width() -> AccessWidth;

        fn stubbed() -> Self;

        fn from_u32(i: u32) -> Self;
        fn as_u32(&self) -> u32;
        fn as_u16(&self) -> u16 {
            self.as_u32() as u16
        }
        fn as_u8(&self) -> u8 {
            self.as_u32() as u8
        }
    }

    impl Addressable for u8 {
        fn width() -> AccessWidth {
            AccessWidth::Byte
        }

        fn from_u32(value: u32) -> Self {
            value as Self
        }

        fn as_u32(&self) -> u32 {
            u32::from(*self)
        }

        fn stubbed() -> Self {
            Self::MAX
        }
    }

    impl Addressable for u16 {
        fn width() -> AccessWidth {
            AccessWidth::Halfword
        }

        fn from_u32(value: u32) -> Self {
            value as Self
        }

        fn as_u32(&self) -> u32 {
            u32::from(*self)
        }

        fn stubbed() -> Self {
            Self::MAX
        }
    }

    impl Addressable for u32 {
        fn width() -> AccessWidth {
            AccessWidth::Word
        }

        fn from_u32(value: Self) -> Self {
            value
        }

        fn as_u32(&self) -> Self {
            *self
        }

        fn stubbed() -> Self {
            Self::MAX
        }
    }
}

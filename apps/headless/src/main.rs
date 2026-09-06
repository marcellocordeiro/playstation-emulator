#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::ffi::OsStr;

use cli::parse_args;
use playstation_core::{PlayStation, constants::BIOS_SIZE};

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .without_time()
        .init();

    let args = parse_args();

    let bios = args
        .bios
        .map(|path| {
            let data = std::fs::read(&path).unwrap();
            let data: Box<[u8; BIOS_SIZE]> =
                data.try_into().expect("size should match the bios size");

            data
        })
        .unwrap();

    let mut rom = args
        .rom
        // Only support exes like amidog tests for now
        .inspect(|path| assert_eq!(path.extension(), Some(OsStr::new("exe"))))
        .map(|path| std::fs::read(&path).unwrap());

    let mut ps = PlayStation::new(bios);

    if let Some(rom) = rom.take() {
        ps.cpu.sideload_exe(&rom);
    }

    // let initial_pc = ps.cpu.regs.pc;

    // for i in 0..50 {
    //     let pc = initial_pc + (i * 4);

    //     let instruction = Instruction(ps.cpu.memory.fetch_instruction(pc));
    //     let decoded = instruction.decoded();
    //     println!("{pc:#010X}  {:08X}  {decoded}", instruction.0);
    //     ps.step();
    // }

    loop {
        ps.step();
    }
}

// fn main() {
//     tracing_subscriber::fmt()
//         .with_max_level(tracing::Level::INFO)
//         .without_time()
//         .init();

//     let bios = include_bytes!("../../../roms/bios/scph1001.bin")
//         .to_vec()
//         .try_into()
//         .unwrap();

//     let mut ps = PlayStation::new(bios);

//     ps.cpu.sideload_amidogs();

//     loop {
//         ps.step();
//     }
// }

mod cli;

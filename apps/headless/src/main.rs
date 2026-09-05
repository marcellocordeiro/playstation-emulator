#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;

use cli::parse_args;
use playstation_core::{PlayStation, constants::BIOS_SIZE};

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .without_time()
        .init();

    let args = parse_args();

    let bios_path = args.bios;

    let bios = bios_path
        .map(PathBuf::from)
        .map(|path| {
            let data = std::fs::read(&path).unwrap();
            let data: Box<[u8; BIOS_SIZE]> =
                data.try_into().expect("size should match the bios size");

            data
        })
        .unwrap();

    let mut ps = PlayStation::new(bios);

    if args.run_amidogs {
        ps.cpu.sideload_amidogs();
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

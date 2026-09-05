#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;

use cli::parse_args;
use playstation_core::{
    PlayStation,
    components::{cpu::instruction::Instruction, memory::MemoryInterface},
    constants::BIOS_SIZE,
};

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

/*
0xBFC00000  3C080013  lui r8, 0x0013
0xBFC00004  3508243F  ori r8, r8, 0x243f
0xBFC00008  3C011F80  lui r1, 0x1f80
0xBFC0000C  AC281010  sw r8, 0x1010(r1)
0xBFC00010  00000000  sll r0, r0, 0
0xBFC00014  24080B88  addiu r8, r0, 0x0b88
0xBFC00018  3C011F80  lui r1, 0x1f80
0xBFC0001C  AC281060  sw r8, 0x1060(r1)
0xBFC00020  00000000  sll r0, r0, 0
0xBFC00024  00000000  sll r0, r0, 0
0xBFC00028  00000000  sll r0, r0, 0
0xBFC0002C  00000000  sll r0, r0, 0
0xBFC00030  00000000  sll r0, r0, 0
0xBFC00034  00000000  sll r0, r0, 0
0xBFC00038  00000000  sll r0, r0, 0
0xBFC0003C  00000000  sll r0, r0, 0
0xBFC00040  00000000  sll r0, r0, 0
0xBFC00044  00000000  sll r0, r0, 0
0xBFC00048  00000000  sll r0, r0, 0
0xBFC0004C  00000000  sll r0, r0, 0
0xBFC00050  00000000  sll r0, r0, 0
0xBFC00054  00000000  sll r0, r0, 0
0xBFC00058  00000000  sll r0, r0, 0
0xBFC0005C  00000000  sll r0, r0, 0
0xBFC00060  00000000  sll r0, r0, 0
0xBFC00064  00000000  sll r0, r0, 0
0xBFC00068  00000000  sll r0, r0, 0
0xBFC0006C  00000000  sll r0, r0, 0
0xBFC00070  0BF00054  j 0x0fc00150
0xBFC00074  00000000  sll r0, r0, 0
0xBFC00078  00000000  sll r0, r0, 0
0xBFC0007C  00000000  sll r0, r0, 0
0xBFC00080  003C0800  sll r1, r28, 0

0xBFC00000  3C080013  lui r8, 0x0013
0xBFC00004  3508243F  ori r8, r8, 0x243f
0xBFC00008  3C011F80  lui r1, 0x1f80
0xBFC0000C  AC281010  sw r8, 0x1010(r1)
0xBFC00010  00000000  sll r0, r0, 0
0xBFC00014  24080B88  addiu r8, r0, 0x0b88
0xBFC00018  3C011F80  lui r1, 0x1f80
0xBFC0001C  AC281060  sw r8, 0x1060(r1)
0xBFC00020  00000000  sll r0, r0, 0
0xBFC00024  00000000  sll r0, r0, 0
0xBFC00028  00000000  sll r0, r0, 0
0xBFC0002C  00000000  sll r0, r0, 0
0xBFC00030  00000000  sll r0, r0, 0
0xBFC00034  00000000  sll r0, r0, 0
0xBFC00038  00000000  sll r0, r0, 0
0xBFC0003C  00000000  sll r0, r0, 0
0xBFC00040  00000000  sll r0, r0, 0
0xBFC00044  00000000  sll r0, r0, 0
0xBFC00048  00000000  sll r0, r0, 0
0xBFC0004C  00000000  sll r0, r0, 0
0xBFC00050  00000000  sll r0, r0, 0
0xBFC00054  00000000  sll r0, r0, 0
0xBFC00058  00000000  sll r0, r0, 0
0xBFC0005C  00000000  sll r0, r0, 0
0xBFC00060  00000000  sll r0, r0, 0
0xBFC00064  00000000  sll r0, r0, 0
0xBFC00068  00000000  sll r0, r0, 0
0xBFC0006C  00000000  sll r0, r0, 0
0xBFC00070  0BF00054  j $0x0fc00150
0xBFC00074  00000000  sll r0, r0, 0
0xBFC00078  00000000  sll r0, r0, 0
0xBFC0007C  00000000  sll r0, r0, 0
0xBFC00080  003C0800  sll r1, r28, 0
*/

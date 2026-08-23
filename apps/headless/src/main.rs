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

    let mut ps = PlayStation::new(*bios);

    loop {
        ps.step();
    }
}

mod cli;

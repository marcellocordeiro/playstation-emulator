#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;

use cli::parse_args;
use playstation_core::constants::{BIOS_SIZE, SCREEN_HEIGHT, SCREEN_WIDTH};

use crate::{
    app::App,
    file_manager::{FileInfoWithSize, FileManager},
};

#[tokio::main]
async fn main() -> eframe::Result {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .without_time()
        .init();

    let args = parse_args();

    let bios_path = args.bios;

    let bios = bios_path.map(PathBuf::from).map(|path| {
        let data = std::fs::read(&path).unwrap();
        let data: Box<[u8; BIOS_SIZE]> = data.try_into().expect("size should match the bios size");

        FileInfoWithSize::<BIOS_SIZE> {
            data: data.into(),
            path,
        }
    });

    let file_manager = FileManager { bios };

    #[allow(clippy::cast_precision_loss)]
    let initial_window_size = egui::vec2(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32);

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size(initial_window_size),
        persist_window: false,
        ..Default::default()
    };

    eframe::run_native(
        "PlayStation",
        native_options,
        Box::new(move |cc| Ok(Box::new(App::new(cc, file_manager)))),
    )
}

mod app;
mod cli;
mod file_manager;
mod gui;
mod sys;

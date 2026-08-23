use std::sync::mpsc::Sender;

use playstation_core::constants::{BIOS_EXTENSIONS, BIOS_EXTENSIONS_DESCRIPTION};

use crate::{file_manager::FileInfo, gui::Event};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FileType {
    Bios,
}

pub fn file_picker_async(file_type: FileType, sender: Sender<Event>) {
    let (extensions_description, extensions) = match file_type {
        FileType::Bios => (BIOS_EXTENSIONS.as_slice(), BIOS_EXTENSIONS_DESCRIPTION),
    };

    let task = rfd::AsyncFileDialog::new()
        .add_filter(extensions, extensions_description)
        .pick_file();

    crate::sys::thread::spawn(async move {
        let file_handle = task.await;

        let Some(file_handle) = file_handle else {
            return;
        };

        let data = file_handle.read().await.into();

        let path = file_handle.path().to_path_buf();

        let file_info = FileInfo { data, path };

        match file_type {
            FileType::Bios => {
                sender.send(Event::BiosSelected(file_info)).unwrap();
            }
        }
    });
}

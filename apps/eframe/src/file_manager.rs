pub use file_info::{FileInfo, FileInfoWithSize};
pub use file_picker_async::{FileType, file_picker_async};
use playstation_core::constants::BIOS_SIZE;

#[derive(Default)]
pub struct FileManager {
    pub bios: Option<FileInfoWithSize<BIOS_SIZE>>,
}

mod file_info;
mod file_picker_async;

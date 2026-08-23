use std::sync::Arc;

#[derive(Debug, Default, Clone)]
pub struct FileInfo {
    pub data: Arc<[u8]>,
    pub path: std::path::PathBuf,
}

#[derive(Debug, Clone)]
pub struct FileInfoWithSize<const SIZE: usize> {
    pub data: Arc<[u8; SIZE]>,
    pub path: std::path::PathBuf,
}

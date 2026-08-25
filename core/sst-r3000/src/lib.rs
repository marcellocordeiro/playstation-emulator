use std::{
    fs::{File, OpenOptions, create_dir_all},
    io::{self},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Test(pub Vec<Entry>);

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Entry {
    pub name: String,
    pub opcode: u32,
    pub opcode_addr: u32,
    pub initial: State,
    pub r#final: State,
    pub cycles: Vec<Cycle>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
    #[serde(rename = "R")]
    pub r: [u32; 32],
    pub hi: u32,
    pub lo: u32,
    #[serde(rename = "EPC")]
    pub epc: u32,
    #[serde(rename = "TAR")]
    pub tar: u32,
    #[serde(rename = "CAUSE")]
    pub cause: u32,
    #[serde(rename = "PC")]
    pub pc: u32,
    pub delay: Delay,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Delay {
    pub load: LoadDelay,
    pub branch: BranchDelay,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoadDelay {
    pub slot: bool,
    pub take: bool,
    pub target: u32,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchDelay {
    pub target: i32,
    pub val: u32,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cycle {
    pub actions: u32,
    pub sz: u32,
    pub addr: i64,
    pub val: i64,
}

pub fn tests_path() -> io::Result<PathBuf> {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));

    manifest
        .join("../../external/SingleStepTests-r3000/v1")
        .canonicalize()
}

pub fn parse_bin<P: AsRef<Path>>(path: P) -> io::Result<Test> {
    let mut file = File::open(path)?;

    Test::from_reader(&mut file)
}

pub fn parse_json_with_cache<P: AsRef<Path>>(path: P) -> io::Result<Test> {
    let path = path.as_ref();

    let file_name = path.file_name().unwrap();

    let cache_path = {
        let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));

        manifest.join(".cached")
    };

    let cached_test = cache_path.join(file_name);

    if !cached_test.exists() {
        cache_test(path, &cached_test);
    }

    let file = File::open(&cached_test)?;
    let test: Test = serde_json::from_reader(file).unwrap();

    Ok(test)
}

fn cache_test<P1: AsRef<Path>, P2: AsRef<Path>>(from: P1, to: P2) {
    let from = from.as_ref();
    let to = to.as_ref();

    let bin_path = from.with_added_extension("bin");
    let test = parse_bin(bin_path).unwrap();

    create_dir_all(to.parent().unwrap()).unwrap();

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(to)
        .expect("Unable to create cached json");

    serde_json::to_writer_pretty(file, &test).expect("Unable to write to cached json");
}

mod bin_parser;
mod traits;

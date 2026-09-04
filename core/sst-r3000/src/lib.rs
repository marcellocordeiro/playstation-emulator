use std::{
    fs::{self, File, OpenOptions},
    io::{self},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tests(pub Vec<Test>);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Test {
    pub name: String,
    pub opcode: u32,
    pub opcode_addr: u32,
    pub initial: State,
    pub r#final: State,
    pub cycles: Vec<Cycle>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Delay {
    pub load: LoadDelay,
    pub branch: BranchDelay,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoadDelay {
    pub target: i32,
    pub val: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchDelay {
    pub slot: bool,
    pub take: bool,
    pub target: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cycle {
    // pub actions: u32,
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

pub fn parse_bin<P: AsRef<Path>>(path: P) -> io::Result<Tests> {
    println!("Opening file: {}", path.as_ref().display());
    let mut file = File::open(path)?;

    Tests::from_reader(&mut file)
}

pub fn parse_json_with_cache<P: AsRef<Path>>(bin_path: P) -> io::Result<Tests> {
    let bin_path = bin_path.as_ref();

    let json_name = bin_path.with_extension("").file_name().unwrap().to_owned();

    let cache_path = {
        let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));

        manifest.join(".cached")
    };

    let cached_test_path = cache_path.join(json_name);

    if !cached_test_path.exists() {
        cache_test(bin_path, &cached_test_path);
    }

    let file = File::open(&cached_test_path)?;
    let test: Tests = serde_json::from_reader(file).unwrap();

    Ok(test)
}

fn cache_test<P1: AsRef<Path>, P2: AsRef<Path>>(bin_path: P1, json_path: P2) {
    let bin_path = bin_path.as_ref();
    let json_path = json_path.as_ref();

    let test = parse_bin(bin_path).unwrap();

    fs::create_dir_all(json_path.parent().unwrap()).unwrap();

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(json_path)
        .expect("Unable to create cached json");

    serde_json::to_writer_pretty(file, &test).expect("Unable to write to cached json");
}

mod bin_parser;
mod diff;
mod traits;

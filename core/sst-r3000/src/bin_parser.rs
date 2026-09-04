use std::io::{self, Read};

use crate::{BranchDelay, Cycle, Delay, LoadDelay, State, Test, Tests, traits::ReadBytesExt as _};

impl Tests {
    pub fn from_reader<R: Read>(reader: &mut R) -> io::Result<Self> {
        let num_tests = usize::try_from(reader.read_le_i32()?).unwrap();

        let mut entries = Vec::new();

        for _ in 0..num_tests {
            let entry = Test::from_reader(reader)?;
            entries.push(entry);
        }

        Ok(Self(entries))
    }
}

impl Test {
    fn from_reader<R: Read>(reader: &mut R) -> io::Result<Self> {
        let name = {
            let mut buffer = [0_u8; 51];
            reader.read_exact(&mut buffer)?;

            let len = buffer[0] as usize;

            let slice = &buffer[1..=len];
            String::from_utf8_lossy(slice).into_owned()
        };

        let opcode = reader.read_le_u32()?;
        let opcode_addr = reader.read_le_u32()?;
        let initial = State::from_reader(reader)?;
        let r#final = State::from_reader(reader)?;

        let cycles = {
            let len = reader.read_le_u32()? as usize;

            let mut cycles = Vec::new();
            cycles.reserve_exact(len);

            for _ in 0..len {
                let item = Cycle::from_reader(reader)?;
                cycles.push(item);
            }

            cycles
        };

        Ok(Self {
            name,
            opcode,
            opcode_addr,
            initial,
            r#final,
            cycles,
        })
    }
}

impl State {
    fn from_reader<R: Read>(reader: &mut R) -> io::Result<Self> {
        let mut r = [0_u32; 32];
        for reg in &mut r {
            *reg = reader.read_le_u32()?;
        }

        assert_eq!(r[0], 0);

        let hi = reader.read_le_u32()?;
        let lo = reader.read_le_u32()?;
        let epc = reader.read_le_u32()?;
        let tar = reader.read_le_u32()?;
        let cause = reader.read_le_u32()?;
        let pc = reader.read_le_u32()?;
        let delay = Delay::from_reader(reader)?;

        Ok(Self {
            r,
            hi,
            lo,
            epc,
            tar,
            cause,
            pc,
            delay,
        })
    }
}

impl Delay {
    fn from_reader<R: Read>(reader: &mut R) -> io::Result<Self> {
        let branch = BranchDelay::from_reader(reader)?;
        let load = LoadDelay::from_reader(reader)?;

        Ok(Self { load, branch })
    }
}

impl LoadDelay {
    fn from_reader<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            target: reader.read_le_i32()?,
            val: reader.read_le_u32()?,
        })
    }
}

impl BranchDelay {
    fn from_reader<R: Read>(reader: &mut R) -> io::Result<Self> {
        let target = reader.read_le_u32()?;
        let slot = reader.read_le_u32()? != 0;
        let take = reader.read_le_u32()? != 0;

        Ok(Self { slot, take, target })
    }
}

impl Cycle {
    fn from_reader<R: Read>(reader: &mut R) -> io::Result<Self> {
        let val = reader.read_le_i64()?;
        let _actions = reader.read_le_u32()?;
        let addr = reader.read_le_i64()?;
        let sz = reader.read_le_u32()?;

        Ok(Self {
            //actions,
            sz,
            addr,
            val,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs::{self, File},
        path::Path,
    };

    use crate::{Tests, parse_bin, parse_json_with_cache, tests_path};

    #[test]
    #[ignore = "no need to run it in the CI"]
    fn validate_python_dumps_with_parser() {
        let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
        let tests_path = manifest
            .join("../../external/SingleStepTests-r3000/v1")
            .canonicalize()
            .unwrap();

        for path in fs::read_dir(tests_path).unwrap() {
            let path = path.unwrap().path();

            if path.extension().unwrap() != "bin" {
                continue;
            }

            let path_json = path.with_extension(""); // Remove ".bin"

            let json: Tests = {
                let file = File::open(path_json).unwrap();
                serde_json::from_reader(file).unwrap()
            };

            let from_bin = parse_bin(path).unwrap();

            assert_eq!(from_bin, json);
        }
    }

    #[test]
    #[ignore = "no need to run it in the CI"]
    fn validate_json_with_cached_json() {
        let tests_path = tests_path().unwrap();

        for path in fs::read_dir(tests_path).unwrap() {
            let path = path.unwrap().path();

            if path.extension().unwrap() != "json" {
                continue;
            }

            let json: Tests = {
                let file = File::open(&path).unwrap();
                serde_json::from_reader(file).unwrap()
            };

            let cached_json = parse_json_with_cache(path).unwrap();

            assert_eq!(cached_json, json);
        }
    }
}

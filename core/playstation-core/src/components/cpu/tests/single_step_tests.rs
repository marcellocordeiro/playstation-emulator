use std::ffi::OsStr;

use itertools::Itertools as _;
use sst_r3000::{State, Test, parse_json_with_cache, tests_path};

use crate::components::cpu::{
    Cpu,
    registers::Registers,
    tests::single_step_tests::test_memory::TestMemory,
};

// SHL (from the tests) == SH (Store Halfword)

fn test_cpu(file_name: &str, test: &Test) {
    let memory = TestMemory::from_opcode_and_cycles(test.opcode_addr, test.opcode, &test.cycles);
    let mut cpu = Cpu::new(memory);

    let initial_regs = Registers::from(test.initial.clone());

    cpu.regs = initial_regs;

    cpu.run_next_instruction();

    let actual = State::from(cpu.regs);
    let expected = &test.r#final;

    let diffs = State::diff(&actual, expected);
    // diffs.append(&mut Cycle::diff(&cpu.memory.cycles.borrow(), &test.cycles));

    assert!(
        diffs.is_empty(),
        "Test `{}` from `{file_name}` failed. The final state does not match the expected result. Diffs: {diffs:#?}",
        test.name
    );
    /*let mut cpu = Cpu::default();
    let mut memory = TestMemory::default();

    cpu.registers = test.initial.clone().into();
    memory.data = test.initial.ram.clone().into_iter().collect();

    cpu.step(&mut memory);

    assert_eq!(
        State::from((cpu.registers, memory.data)),
        test.r#final,
        "Test `{}` from `{file_name}` failed. The final state does not match the expected result.",
        test.name
    );

    assert_eq!(
        *memory.logs, test.cycles,
        "Test `{}` from `{file_name}` failed. The trace does not match the expected result.",
        test.name
    );*/
}

fn get_test_files() -> Vec<std::fs::DirEntry> {
    let path = tests_path().unwrap();

    println!("Path: {}", path.display());

    let files = std::fs::read_dir(path)
        .unwrap()
        .filter_map(|file| {
            let file = file.unwrap();

            (file.path().extension() == Some(OsStr::new("bin"))).then_some(file)
        })
        .sorted_by(|a, b| a.path().cmp(&b.path()))
        .collect::<Vec<_>>();

    assert_eq!(files.len(), 55);

    files
}

#[test]
fn single_step_tests() {
    let files = get_test_files();

    for file in &files {
        let file_path = file.path(); // ADD.json.bin -> ADD.json
        let file_name = file_path
            .with_extension("")
            .with_extension("")
            .file_stem()
            .unwrap()
            .to_owned()
            .to_str()
            .unwrap()
            .to_owned();

        let tests = parse_json_with_cache(&file_path).unwrap();

        for test in tests.0 {
            test_cpu(&file_name, &test);
        }
    }
}

#[test]
#[ignore = "manual only, never fails"]
fn one_shot() {
    #[derive(Default)]
    struct TestResult {
        passed: i32,
        total: i32,
    }

    let files = get_test_files();

    std::panic::set_hook(Box::new(|_| {}));

    for file in &files {
        let file_path = file.path();
        let file_name = file_path
            .with_extension("")
            .with_extension("")
            .file_stem()
            .unwrap()
            .to_owned()
            .to_str()
            .unwrap()
            .to_owned();

        let tests = parse_json_with_cache(&file_path).unwrap();

        let mut test_result = TestResult::default();

        for test in tests.0 {
            let result = std::panic::catch_unwind(|| {
                test_cpu(&file_name, &test);
            });

            test_result.passed += i32::from(result.is_ok());
            test_result.total += 1;
        }

        println!(
            "{file_name}: {} out of {}",
            test_result.passed, test_result.total
        );
    }
}

#[test]
#[ignore = "manual only"]
fn with_selection() {
    let selection = "LWL.json.bin";
    let file_path = tests_path().unwrap().join(selection);

    let file_name = file_path
        .with_extension("")
        .with_extension("")
        .file_stem()
        .unwrap()
        .to_owned()
        .to_str()
        .unwrap()
        .to_owned();

    let tests = parse_json_with_cache(&file_path).unwrap();

    if cfg!(true) {
        for test in tests.0 {
            test_cpu(&file_name, &test);
        }
    } else {
        let test = &tests.0[0x002];
        println!("Test name: {}", test.name);
        test_cpu(&file_name, test);
    }
}

mod structs;
mod test_memory;
mod test_ram;

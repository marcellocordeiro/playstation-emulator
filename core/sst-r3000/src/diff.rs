use crate::{BranchDelay, LoadDelay, State};

#[derive(Debug, Clone, Copy)]
pub enum StateDiff {
    Register { index: usize, values: (u32, u32) },
    Hi(u32, u32),
    Lo(u32, u32),
    Epc(u32, u32),
    Tar(u32, u32),
    Cause(u32, u32),
    Pc(u32, u32),
    LoadDelay(LoadDelay, LoadDelay),
    BranchDelay(BranchDelay, BranchDelay),
}

impl State {
    #[must_use]
    pub fn diff(actual: &Self, expected: &Self) -> Vec<StateDiff> {
        let mut diffs = Vec::new();

        for (index, (lhs, rhs)) in actual.r.into_iter().zip(expected.r).enumerate() {
            if lhs != rhs {
                diffs.push(StateDiff::Register {
                    index,
                    values: (lhs, rhs),
                });
            }
        }

        if actual.hi != expected.hi {
            diffs.push(StateDiff::Hi(actual.hi, expected.hi));
        }

        if actual.lo != expected.lo {
            diffs.push(StateDiff::Lo(actual.lo, expected.lo));
        }

        if actual.epc != expected.epc {
            diffs.push(StateDiff::Epc(actual.epc, expected.epc));
        }

        // if actual.cause != expected.cause {
        //     diffs.push(StateDiff::Cause(actual.cause, expected.cause));
        // }

        if actual.pc != expected.pc {
            diffs.push(StateDiff::Pc(actual.pc, expected.pc));
        }

        if actual.delay.load != expected.delay.load {
            diffs.push(StateDiff::LoadDelay(actual.delay.load, expected.delay.load));
        }

        if actual.delay.branch != expected.delay.branch {
            diffs.push(StateDiff::BranchDelay(
                actual.delay.branch,
                expected.delay.branch,
            ));
        }

        diffs
    }
}

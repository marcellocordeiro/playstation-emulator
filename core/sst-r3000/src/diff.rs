use crate::{BranchDelay, Cycle, LoadDelay, State};

#[derive(Debug, Clone)]
pub enum Diff {
    Register { index: usize, values: (u32, u32) },
    Hi(u32, u32),
    Lo(u32, u32),
    Epc(u32, u32),
    Tar(u32, u32),
    Cause(u32, u32),
    Pc(u32, u32),
    LoadDelay(LoadDelay, LoadDelay),
    BranchDelay(BranchDelay, BranchDelay),
    Cycles(Vec<Cycle>, Vec<Cycle>),
}

impl State {
    #[must_use]
    pub fn diff(actual: &Self, expected: &Self) -> Vec<Diff> {
        let mut diffs = Vec::new();

        for (index, (lhs, rhs)) in actual.r.into_iter().zip(expected.r).enumerate() {
            if lhs != rhs {
                diffs.push(Diff::Register {
                    index,
                    values: (lhs, rhs),
                });
            }
        }

        if actual.hi != expected.hi {
            diffs.push(Diff::Hi(actual.hi, expected.hi));
        }

        if actual.lo != expected.lo {
            diffs.push(Diff::Lo(actual.lo, expected.lo));
        }

        if actual.epc != expected.epc {
            diffs.push(Diff::Epc(actual.epc, expected.epc));
        }

        // if actual.cause != expected.cause {
        //     diffs.push(StateDiff::Cause(actual.cause, expected.cause));
        // }

        if actual.pc != expected.pc {
            diffs.push(Diff::Pc(actual.pc, expected.pc));
        }

        if actual.delay.load != expected.delay.load {
            diffs.push(Diff::LoadDelay(
                actual.delay.load.clone(),
                expected.delay.load.clone(),
            ));
        }

        if actual.delay.branch != expected.delay.branch {
            diffs.push(Diff::BranchDelay(
                actual.delay.branch.clone(),
                expected.delay.branch.clone(),
            ));
        }

        diffs
    }
}

impl Cycle {
    pub fn diff(actual: &Vec<Self>, expected: &Vec<Self>) -> Vec<Diff> {
        if actual != expected {
            vec![Diff::Cycles(actual.clone(), expected.clone())]
        } else {
            vec![]
        }
    }
}

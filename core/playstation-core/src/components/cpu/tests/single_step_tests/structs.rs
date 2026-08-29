use sst_r3000::{BranchDelay, Delay, LoadDelay, State};

use crate::components::cpu::{instruction::RegisterIndex, registers::Registers};

impl From<State> for Registers {
    fn from(value: State) -> Self {
        let State {
            r,
            hi,
            lo,
            epc: _,
            tar: _,
            cause: _,
            pc,
            delay,
        } = value;

        let delayed_load = {
            let LoadDelay { target, val } = delay.load;

            if target == -1 {
                None
            } else {
                Some((RegisterIndex(target as u32), val))
            }
        };

        let delayed_branch = {
            let BranchDelay { slot, take, target } = delay.branch;

            if slot { Some((target, take)) } else { None }
        };

        Self {
            r,
            pc,
            hi,
            lo,
            delayed_load,
            delayed_branch,
            next_pc: delayed_branch.map_or(pc.wrapping_add(4), |b| {
                if b.1 { b.0 } else { pc.wrapping_add(4) }
            }),
            ..Default::default()
        }
    }
}

impl From<Registers> for State {
    fn from(value: Registers) -> Self {
        let Registers {
            r,
            pc,
            hi,
            lo,
            delayed_load,
            delayed_load_next: _,
            delayed_branch,
            next_pc: _,
        } = value;

        let load = {
            let (target, val) =
                delayed_load.map_or((-1, 0), |(target, val)| (target.0 as i32, val));

            LoadDelay { target, val }
        };

        let branch = {
            let slot = delayed_branch.is_some();
            let (target, take) = delayed_branch.unwrap_or((0, false));

            BranchDelay { slot, take, target }
        };

        Self {
            r,
            hi,
            lo,
            epc: 0,
            tar: 0,
            cause: 0,
            pc,
            delay: Delay { load, branch },
        }
    }
}

use sst_r3000::{BranchDelay, Delay, LoadDelay, State};

use crate::components::cpu::{
    cop0::{CauseRegister, Cop0, StatusRegister},
    instruction::RegisterIndex,
    registers::Registers,
};

impl From<State> for Registers {
    fn from(value: State) -> Self {
        let State {
            r,
            hi,
            lo,
            epc,
            tar: _,
            cause,
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

        let cop0 = {
            let sr = StatusRegister::default();
            let cause = CauseRegister::from(cause);

            Cop0 { sr, cause, epc }
        };

        Self {
            r,
            pc,
            hi,
            lo,
            delayed_load,
            delayed_branch,
            cop0,
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
            cop0,
        } = value;

        let Cop0 { sr: _, cause, epc } = cop0;
        let cause = cause.read();

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
            epc,
            tar: 0,
            cause,
            pc,
            delay: Delay { load, branch },
        }
    }
}

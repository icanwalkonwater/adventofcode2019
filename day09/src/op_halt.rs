use crate::{MachineState, Run};

pub struct OpHalt;

impl OpHalt {
    pub fn new() -> Self {
        OpHalt {}
    }
}

impl Default for OpHalt {
    fn default() -> Self {
        OpHalt::new()
    }
}

impl Run for OpHalt {
    fn run(&self, _: &mut [i64], state: &mut MachineState) {
        state.halt()
    }
}

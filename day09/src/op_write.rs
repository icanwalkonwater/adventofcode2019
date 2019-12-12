use crate::{MachineState, Run};

pub struct OpWrite {
    ptr_val: usize,
}

impl OpWrite {
    pub fn new(ptr_val: usize) -> OpWrite {
        OpWrite { ptr_val }
    }
}

impl Run for OpWrite {
    fn run(&self, memory: &mut [i64], state: &mut MachineState) {
        let value = memory[self.ptr_val];
        state.push_output(value);

        // Move 2 forward
        state.jmp_relative(2);
    }
}

use crate::{MachineState, Run};

pub struct OpBaseSet {
    ptr_val: usize,
}

impl OpBaseSet {
    pub fn new(ptr_val: usize) -> Self {
        OpBaseSet { ptr_val }
    }
}

impl Run for OpBaseSet {
    fn run(&self, memory: &mut [i64], state: &mut MachineState) {
        state.bp_update(memory[self.ptr_val] as isize);

        // Move 2 forward
        state.jmp_relative(2);
    }
}

use crate::{MachineState, Run};

pub struct OpRead {
    ptr_res: usize,
}

impl OpRead {
    pub fn new(ptr_res: usize) -> OpRead {
        OpRead { ptr_res }
    }
}

impl Run for OpRead {
    fn run(&self, memory: &mut [i64], state: &mut MachineState) {
        let read = state.pop_input();
        memory[self.ptr_res] = read;

        // Move 2 forward
        state.jmp_relative(2);
    }
}

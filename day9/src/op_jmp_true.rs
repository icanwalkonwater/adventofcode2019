use crate::{MachineState, Run};

pub struct OpJmpTrue {
    ptr_val: usize,
    ptr_to: usize,
}

impl OpJmpTrue {
    pub fn new(ptr_val: usize, ptr_to: usize) -> OpJmpTrue {
        OpJmpTrue { ptr_val, ptr_to }
    }
}

impl Run for OpJmpTrue {
    fn run(&self, memory: &mut [i64], state: &mut MachineState) {
        if memory[self.ptr_val] != 0 {
            // Jump
            state.jmp(memory[self.ptr_to] as usize);
        } else {
            // Move 3 forward
            state.jmp_relative(3);
        }
    }
}

use crate::{MachineState, Run};

pub struct OpLess {
    ptr_lhs: usize,
    ptr_rhs: usize,
    ptr_res: usize,
}

impl OpLess {
    pub fn new(ptr_lhs: usize, ptr_rhs: usize, ptr_res: usize) -> OpLess {
        OpLess {
            ptr_lhs,
            ptr_rhs,
            ptr_res,
        }
    }
}

impl Run for OpLess {
    fn run(&self, memory: &mut [i64], state: &mut MachineState) {
        if memory[self.ptr_lhs] < memory[self.ptr_rhs] {
            memory[self.ptr_res] = 1;
        } else {
            memory[self.ptr_res] = 0;
        }

        // Move 4 forward
        state.jmp_relative(4);
    }
}

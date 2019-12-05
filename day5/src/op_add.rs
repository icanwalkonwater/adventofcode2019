use crate::Run;

pub struct OpAdd {
    ptr_lhs: usize,
    ptr_rhs: usize,
    ptr_res: usize,
}

impl OpAdd {
    pub fn new(ptr_lhs: usize, ptr_rhs: usize, ptr_res: usize) -> OpAdd {
        OpAdd {
            ptr_lhs,
            ptr_rhs,
            ptr_res,
        }
    }
}

impl Run for OpAdd {
    fn run(&self, memory: &mut [i32], ip: usize) -> usize {
        memory[self.ptr_res] = memory[self.ptr_lhs] + memory[self.ptr_rhs];

        // Move 4 forward
        ip + 4
    }
}

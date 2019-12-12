use crate::Run;

pub struct OpEq {
    ptr_lhs: usize,
    ptr_rhs: usize,
    ptr_res: usize,
}

impl OpEq {
    pub fn new(ptr_lhs: usize, ptr_rhs: usize, ptr_res: usize) -> OpEq {
        OpEq {
            ptr_lhs,
            ptr_rhs,
            ptr_res,
        }
    }
}

impl Run for OpEq {
    fn run(&self, memory: &mut [i32], ip: usize) -> usize {
        if memory[self.ptr_lhs] == memory[self.ptr_rhs] {
            memory[self.ptr_res] = 1;
        } else {
            memory[self.ptr_res] = 0;
        }

        // Move 4 forward
        ip + 4
    }
}

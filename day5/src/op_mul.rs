use crate::Run;

pub struct OpMul {
    ptr_lhs: usize,
    ptr_rhs: usize,
    ptr_res: usize,
}

impl OpMul {
    pub fn new(ptr_lhs: usize, ptr_rhs: usize, ptr_res: usize) -> OpMul {
        OpMul {
            ptr_lhs,
            ptr_rhs,
            ptr_res,
        }
    }
}

impl Run for OpMul {
    fn run(&self, memory: &mut [i32], ip: usize) -> usize {
        memory[self.ptr_res] = memory[self.ptr_lhs] * memory[self.ptr_rhs];

        // Move 4 forward
        ip + 4
    }
}

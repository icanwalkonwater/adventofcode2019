use crate::Run;

pub struct OpJmpFalse {
    ptr_val: usize,
    ptr_to: usize,
}

impl OpJmpFalse {
    pub fn new(ptr_val: usize, ptr_to: usize) -> OpJmpFalse {
        OpJmpFalse { ptr_val, ptr_to }
    }
}

impl Run for OpJmpFalse {
    fn run(&self, memory: &mut [i32], ip: usize) -> usize {
        if memory[self.ptr_val] == 0 {
            // Jump
            memory[self.ptr_to] as usize
        } else {
            // Move 3 forward
            ip + 3
        }
    }
}

use crate::Run;

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
    fn run(&self, memory: &mut [i32], ip: usize) -> usize {
        if memory[self.ptr_val] != 0 {
            // Jump
            memory[self.ptr_to] as usize
        } else {
            // Move 3 forward
            ip + 3
        }
    }
}

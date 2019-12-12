use crate::Run;
use std::collections::VecDeque;

pub struct OpWrite {
    ptr_val: usize,
}

impl OpWrite {
    pub fn new(ptr_val: usize) -> OpWrite {
        OpWrite { ptr_val }
    }
}

impl Run for OpWrite {
    fn run(&self, _: &mut [i32], _: usize) -> usize {
        panic!("This opcode requires an output");
    }

    fn run_io(
        &self,
        memory: &mut [i32],
        _: &mut VecDeque<i32>,
        output: &mut VecDeque<i32>,
        ip: usize,
    ) -> usize {
        let value = memory[self.ptr_val];

        output.push_back(value);

        // Move 2 forward
        ip + 2
    }
}

use crate::Run;

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
        _: &mut Vec<i32>,
        output: &mut Vec<i32>,
        ip: usize,
    ) -> usize {
        let value = memory[self.ptr_val];

        output.push(value);

        // Move 2 forward
        ip + 2
    }
}

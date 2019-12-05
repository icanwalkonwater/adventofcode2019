use crate::Run;

pub struct OpRead {
    ptr_res: usize,
}

impl OpRead {
    pub fn new(ptr_res: usize) -> OpRead {
        OpRead { ptr_res }
    }
}

impl Run for OpRead {
    fn run(&self, _: &mut [i32], _: usize) -> usize {
        panic!("This opcode requires an input !");
    }

    fn run_io(
        &self,
        memory: &mut [i32],
        input: &mut Vec<i32>,
        _: &mut Vec<i32>,
        ip: usize,
    ) -> usize {
        let read = input.pop().expect("Input exhausted !");

        memory[self.ptr_res] = read;

        // Move 2 forward
        ip + 2
    }
}

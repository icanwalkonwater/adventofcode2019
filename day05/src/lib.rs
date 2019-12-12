use std::collections::VecDeque;

mod op_add;
mod op_eq;
mod op_jmp_false;
mod op_jmp_true;
mod op_less;
mod op_mul;
mod op_read;
mod op_write;
mod param_iterator;
pub mod parser;

pub use op_add::OpAdd;
pub use op_eq::OpEq;
pub use op_jmp_false::OpJmpFalse;
pub use op_jmp_true::OpJmpTrue;
pub use op_less::OpLess;
pub use op_mul::OpMul;
pub use op_read::OpRead;
pub use op_write::OpWrite;

pub trait Run {
    fn run(&self, memory: &mut [i32], ip: usize) -> usize;

    fn run_io(
        &self,
        memory: &mut [i32],
        _ /*input*/: &mut VecDeque<i32>,
        _ /*output*/: &mut VecDeque<i32>,
        ip: usize,
    ) -> usize {
        self.run(memory, ip)
    }
}

pub fn run(mut memory: Vec<i32>, input: Vec<i32>) -> i32 {
    // Convert input
    let mut input = VecDeque::from(input);

    // Instruction pointer
    let mut ip = 0;

    // Output buffer
    let mut output = VecDeque::new();

    // Run until a None is found
    while let Some(instr) = parser::parse_next_instr_to_runnable(&memory[ip..], ip) {
        ip = instr.run_io(&mut memory, &mut input, &mut output, ip);
    }

    // Return the last thing outputted
    *output.back().expect("No output")
}

mod op_add;
mod op_eq;
mod op_jmp_false;
mod op_jmp_true;
mod op_less;
mod op_mul;
mod op_read;
mod op_write;
mod param_iterator;
mod parser;

pub trait Run {
    fn run(&self, memory: &mut [i32], ip: usize) -> usize;

    fn run_io(&self, memory: &mut [i32], _: &mut Vec<i32>, _: &mut Vec<i32>, ip: usize) -> usize {
        self.run(memory, ip)
    }
}

pub fn run(mut memory: Vec<i32>, mut input: Vec<i32>) -> i32 {
    // Instruction pointer
    let mut ip = 0;

    // Output buffer
    let mut output = Vec::new();

    // Run until a None is found
    while let Some(instr) = parser::parse_next_instr_to_runnable(&memory[ip..], ip) {
        ip = instr.run_io(&mut memory, &mut input, &mut output, ip);
        //println!("# IP: {}", ip);
    }

    // Return the last output
    *output.last().unwrap_or(&0)
}

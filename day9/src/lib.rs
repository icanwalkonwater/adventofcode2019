mod machine_state;
mod op_add;
mod op_base_set;
mod op_eq;
mod op_jmp_false;
mod op_jmp_true;
mod op_less;
mod op_mul;
mod op_read;
mod op_write;
mod param_iterator;
pub mod parser;

pub use machine_state::MachineState;
pub use op_add::OpAdd;
pub use op_base_set::OpBaseSet;
pub use op_eq::OpEq;
pub use op_jmp_false::OpJmpFalse;
pub use op_jmp_true::OpJmpTrue;
pub use op_less::OpLess;
pub use op_mul::OpMul;
pub use op_read::OpRead;
pub use op_write::OpWrite;

pub trait Run {
    fn run(&self, memory: &mut [i64], state: &mut MachineState);
}

pub fn start_int_machine(memory: Vec<i64>, input: Vec<i64>) -> MachineState {
    // Create memory region of 0x8000 bytes (0x1000 64bits integers)
    let mut memory_region = [0; 0x1000];
    // Copy the provided memory
    for (i, data) in memory.into_iter().enumerate() {
        memory_region[i] = data;
    }

    // Create machine state
    let mut state = MachineState::default();
    for x in input {
        state.push_input(x);
    }

    // Run until a None is found
    while let Some(instr) =
        parser::parse_next_instr_to_runnable(&memory_region[state.ip()..], state.ip(), state.bp())
    {
        instr.run(&mut memory_region, &mut state);
    }

    // A halt has been reached
    state.halt();

    // Return the whole state
    state
}

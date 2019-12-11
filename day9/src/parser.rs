use crate::op_add::OpAdd;
use crate::op_eq::OpEq;
use crate::op_jmp_false::OpJmpFalse;
use crate::op_jmp_true::OpJmpTrue;
use crate::op_less::OpLess;
use crate::op_mul::OpMul;
use crate::op_read::OpRead;
use crate::op_write::OpWrite;
use crate::param_iterator::ParamHolder;
use crate::param_iterator::ParamMode::{Immediate, Position, Relative};
use crate::{OpBaseSet, OpHalt, Run};

pub fn parse_next_instr_to_runnable(snap: &[i64], offset: usize, bp: usize) -> Box<dyn Run> {
    let (opcode, param_modes) = decode_opcode(snap[0]);

    match opcode {
        1 => {
            // Add
            let params = parse_params_to_ptr(&snap[1..=3], param_modes, offset + 1, bp);
            Box::new(OpAdd::new(params[0], params[1], params[2]))
        }
        2 => {
            // Mul
            let params = parse_params_to_ptr(&snap[1..=3], param_modes, offset + 1, bp);
            Box::new(OpMul::new(params[0], params[1], params[2]))
        }
        3 => {
            // Read
            let ptr_res = parse_params_to_ptr(&snap[1..=1], param_modes, offset + 1, bp)[0];
            Box::new(OpRead::new(ptr_res))
        }
        4 => {
            // Write
            let ptr_val = parse_params_to_ptr(&snap[1..=1], param_modes, offset + 1, bp)[0];
            Box::new(OpWrite::new(ptr_val))
        }
        5 => {
            // If true
            let params = parse_params_to_ptr(&snap[1..=2], param_modes, offset + 1, bp);
            Box::new(OpJmpTrue::new(params[0], params[1]))
        }
        6 => {
            // If false
            let params = parse_params_to_ptr(&snap[1..=2], param_modes, offset + 1, bp);
            Box::new(OpJmpFalse::new(params[0], params[1]))
        }
        7 => {
            // Less than
            let params = parse_params_to_ptr(&snap[1..=3], param_modes, offset + 1, bp);
            Box::new(OpLess::new(params[0], params[1], params[2]))
        }
        8 => {
            // Equal
            let params = parse_params_to_ptr(&snap[1..=3], param_modes, offset + 1, bp);
            Box::new(OpEq::new(params[0], params[1], params[2]))
        }
        9 => {
            // Base set
            let ptr_val = parse_params_to_ptr(&snap[1..=2], param_modes, offset + 1, bp)[0];
            Box::new(OpBaseSet::new(ptr_val))
        }
        99 => {
            // Halt
            Box::new(OpHalt::new())
        }
        _ => panic!("Unknown opcode"),
    }
}

fn decode_opcode(full_opcode: i64) -> (u64, Vec<char>) {
    let full_opcode = format!("{:02}", full_opcode);

    let opcode: u64 = full_opcode[full_opcode.len() - 2..]
        .parse()
        .expect("Invalid opcode");

    let parts: Vec<char> = full_opcode.chars().rev().skip(2).collect();

    (opcode, parts)
}

fn parse_params_to_ptr(
    memory: &[i64],
    param_modes: Vec<char>,
    offset: usize,
    bp: usize,
) -> Vec<usize> {
    // Create a Vec to hold the final pointers
    let mut ptrs = Vec::with_capacity(memory.len());
    // An iterator over the parameter modes
    let mut modes_it = ParamHolder::from(param_modes);

    // Compute each parameter to point to its correct value in memory
    for (i, &cell) in memory.iter().enumerate() {
        let mode = modes_it.next().unwrap(); // Can't be None
        let ptr = match mode {
            Position => cell as usize,
            Immediate => offset + i,
            Relative => (bp as i64 + cell) as usize,
        };

        ptrs.push(ptr);
    }

    ptrs
}

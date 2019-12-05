use crate::op_add::OpAdd;
use crate::op_eq::OpEq;
use crate::op_jmp_false::OpJmpFalse;
use crate::op_jmp_true::OpJmpTrue;
use crate::op_less::OpLess;
use crate::op_mul::OpMul;
use crate::op_read::OpRead;
use crate::op_write::OpWrite;
use crate::param_iterator::ParamHolder;
use crate::param_iterator::ParamMode::{Immediate, Position};
use crate::Run;

pub fn parse_next_instr_to_runnable(snap: &[i32], offset: usize) -> Option<Box<dyn Run>> {
    let (opcode, param_modes) = decode_opcode(snap[0]);

    match opcode {
        1 => {
            // Add
            let params = parse_params_to_ptr(&snap[1..=3], param_modes, offset + 1);

            Some(Box::new(OpAdd::new(params[0], params[1], params[2])))
        }
        2 => {
            // Mul
            let params = parse_params_to_ptr(&snap[1..=3], param_modes, offset + 1);

            Some(Box::new(OpMul::new(params[0], params[1], params[2])))
        }
        3 => {
            // Read
            let ptr_res = parse_params_to_ptr(&snap[1..=1], param_modes, offset + 1)[0];

            Some(Box::new(OpRead::new(ptr_res)))
        }
        4 => {
            // Write
            let ptr_val = parse_params_to_ptr(&snap[1..=1], param_modes, offset + 1)[0];

            Some(Box::new(OpWrite::new(ptr_val)))
        }
        5 => {
            // If true
            let params = parse_params_to_ptr(&snap[1..=2], param_modes, offset + 1);

            Some(Box::new(OpJmpTrue::new(params[0], params[1])))
        }
        6 => {
            // If false
            let params = parse_params_to_ptr(&snap[1..=2], param_modes, offset + 1);

            Some(Box::new(OpJmpFalse::new(params[0], params[1])))
        }
        7 => {
            let params = parse_params_to_ptr(&snap[1..=3], param_modes, offset + 1);

            Some(Box::new(OpLess::new(params[0], params[1], params[2])))
        }
        8 => {
            let params = parse_params_to_ptr(&snap[1..=3], param_modes, offset + 1);

            Some(Box::new(OpEq::new(params[0], params[1], params[2])))
        }
        99 => None, // Halt
        _ => panic!("Unknown opcode"),
    }
}

fn decode_opcode(full_opcode: i32) -> (u32, Vec<char>) {
    let full_opcode = format!("{:02}", full_opcode);

    let opcode: u32 = full_opcode[full_opcode.len() - 2..]
        .parse()
        .expect("Invalid opcode");

    let parts: Vec<char> = full_opcode.chars().rev().skip(2).collect();

    //println!("Full opcode: {}", full_opcode);
    //println!("-- opcode: {}", opcode);
    //println!("-- parts: {:?}", parts);

    (opcode, parts)
}

fn parse_params_to_ptr(memory: &[i32], param_modes: Vec<char>, offset: usize) -> Vec<usize> {
    let mut ptrs = Vec::with_capacity(memory.len());
    let mut modes_it = ParamHolder::from(param_modes);

    for (i, &cell) in memory.iter().enumerate() {
        let mode = modes_it.next().unwrap(); // Can't be None
        let ptr = match mode {
            Position => cell as usize,
            Immediate => offset + i,
        };

        //println!("-- param {}: {:?} => {}", i, mode, ptr);

        ptrs.push(ptr);
    }

    ptrs
}

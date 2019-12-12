use std::fs;

fn main() {
    // Load and parse the file to create the initial memory
    let memory: Vec<u32> = fs::read_to_string("./input/day2.txt")
        .expect("Please put your input under ./input/day2.txt")
        .trim()
        .split(',')
        .map(|num| num.parse::<u32>().unwrap())
        .collect();

    // Load part 2 input
    let expected = fs::read_to_string("./input/day2-2.txt")
        .map(|content| content.trim().parse::<u32>().unwrap());

    // Part 1
    let res_1202 = run(memory.clone(), 12, 2);

    println!("Day 2:");
    println!("-- Part 1: {}", res_1202);

    // Part 2
    if let Ok(expected) = expected {
        if let Some((noun, verb)) = brute_force(&memory, expected) {
            println!("-- Part 2: {:02}{:02}", noun, verb);
        } else {
            println!("-- Part 2: Failure !");
        }
    } else {
        println!("-- Part 2: No input provided");
    }
}

fn brute_force(memory: &Vec<u32>, expected: u32) -> Option<(u32, u32)> {
    for noun in 0..99 {
        for verb in 0..99 {
            let result = run(memory.clone(), noun, verb);
            if result == expected {
                return Some((noun, verb));
            }
        }
    }

    None
}

fn run(mut memory: Vec<u32>, noun: u32, verb: u32) -> u32 {
    // Instruction pointer
    let mut rip = 0;

    // Init parameters
    memory[1] = noun;
    memory[2] = verb;

    // Execute until the programm halts
    loop {
        match memory[rip] {
            1 => {
                opcode_add(&mut memory, rip);
                rip += 4;
            }
            2 => {
                opcode_mul(&mut memory, rip);
                rip += 4;
            }
            99 => break, // Halt
            _ => panic!("Unknown opcode !"),
        }
    }

    memory[0]
}

fn opcode_add(memory: &mut [u32], rip: usize) {
    let ptr_op1 = memory[rip + 1] as usize;
    let ptr_op2 = memory[rip + 2] as usize;
    let ptr_res = memory[rip + 3] as usize;

    memory[ptr_res] = memory[ptr_op1] + memory[ptr_op2];
}

fn opcode_mul(memory: &mut [u32], rip: usize) {
    let ptr_op1 = memory[rip + 1] as usize;
    let ptr_op2 = memory[rip + 2] as usize;
    let ptr_res = memory[rip + 3] as usize;

    memory[ptr_res] = memory[ptr_op1] * memory[ptr_op2];
}

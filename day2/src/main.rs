use std::fs;

fn main() {
    // Load file content
    let input = fs::read_to_string("./input/day2.txt")
        .expect("Please put your input under ./input/day2.txt");

    // Parse the file to create the initial memory
    let memory: Vec<u32> = input
        .trim()
        .split(',')
        .map(|num| num.parse::<u32>().unwrap())
        .collect();

    // Bruteforce noun & verb
    let expected = 19690720; // <- input part2 here

    'outer: for noun in 0..99 {
        for verb in 0..99 {
            let result = run(memory.clone(), noun, verb);
            if result == expected {
                println!("{:02}{:02}", noun, verb);
                break 'outer;
            }
        }
    }
}

fn run(mut memory: Vec<u32>, noun: u32, verb: u32) -> u32 {
    // Clone the program memory
    //let mut memory = memory.clone();
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

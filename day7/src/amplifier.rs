use std::collections::VecDeque;

use day5::parser;

#[derive(Debug)]
pub struct Amplifier {
    phase: i32,
    memory: Vec<i32>,
    ip: usize,
    halted: bool,
    input: VecDeque<i32>,
    output: VecDeque<i32>,
}

impl Amplifier {
    pub fn new(memory: Vec<i32>, phase: u8) -> Self {
        let mut input = VecDeque::new();
        input.push_back(phase as i32);

        Amplifier {
            phase: phase as i32,
            memory,
            ip: 0,
            halted: false,
            input,
            output: VecDeque::new(),
        }
    }

    pub fn halted(&self) -> bool {
        self.halted
    }

    pub fn push_signal(&mut self, data: i32) {
        self.input.push_back(data);
    }

    pub fn pop_signal(&mut self) -> Option<i32> {
        self.output.pop_front()
    }

    pub fn resume(&mut self) {
        let out_len = self.output.len();

        // Run until an output is produced or the program halts
        if !self.halted {
            loop {
                if let Some(instr) =
                    parser::parse_next_instr_to_runnable(&self.memory[self.ip..], self.ip)
                {
                    self.ip =
                        instr.run_io(&mut self.memory, &mut self.input, &mut self.output, self.ip);

                    if self.output.len() > out_len {
                        // Write found
                        break;
                    }
                } else {
                    // Halt
                    self.halted = true;
                    break;
                }
            }
        }
    }

    pub fn wait_termination(&mut self) {
        if !self.halted {
            while let Some(instr) =
                parser::parse_next_instr_to_runnable(&self.memory[self.ip..], self.ip)
            {
                self.ip =
                    instr.run_io(&mut self.memory, &mut self.input, &mut self.output, self.ip);
            }

            self.halted = true;
        }
    }
}

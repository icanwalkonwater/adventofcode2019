use std::collections::VecDeque;

#[derive(Debug)]
pub struct MachineState {
    halted: bool,
    ip: usize,
    bp: usize,
    input: VecDeque<i64>,
    output: VecDeque<i64>,
}

impl Default for MachineState {
    fn default() -> Self {
        MachineState::new()
    }
}

impl MachineState {
    pub fn new() -> Self {
        MachineState {
            halted: false,
            ip: 0,
            bp: 0,
            input: VecDeque::new(),
            output: VecDeque::new(),
        }
    }

    pub fn halted(&self) -> bool {
        self.halted
    }

    pub fn halt(&mut self) {
        self.halted = true;
    }

    pub fn ip(&self) -> usize {
        self.ip
    }

    pub fn jmp_relative(&mut self, delta: usize) {
        self.ip += delta;
    }

    pub fn jmp(&mut self, to: usize) {
        self.ip = to;
    }

    pub fn bp(&self) -> usize {
        self.bp
    }

    pub fn bp_update(&mut self, delta: isize) {
        self.bp = (self.bp as isize + delta) as usize
    }

    pub fn push_input(&mut self, data: i64) {
        self.input.push_back(data);
    }

    pub fn pop_input(&mut self) -> i64 {
        self.input.pop_front().expect("Input exhausted")
    }

    pub fn output(&self) -> &VecDeque<i64> {
        &self.output
    }

    pub fn output_mut(&mut self) -> &mut VecDeque<i64> {
        &mut self.output
    }

    pub fn into_output(self) -> Vec<i64> {
        self.output.into_iter().collect()
    }

    pub fn push_output(&mut self, data: i64) {
        self.output.push_back(data);
    }
}

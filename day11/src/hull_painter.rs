use day9::MachineState;
use day9::parser;

use crate::Color;
use crate::hull_painter::Direction::{Down, Left, Right, Up};

#[derive(Copy, Clone, Debug)]
enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

impl From<isize> for Direction {
    fn from(v: isize) -> Self {
        match v {
            0 => Up,
            1 => Right,
            2 => Down,
            3 => Left,
            _ => panic!("Invalid direction")
        }
    }
}

#[derive(Debug)]
pub struct HullPainter {
    memory_region: Vec<i64>,
    state: MachineState,
    position: (usize, usize),
    direction: Direction,
}

impl HullPainter {
    pub fn new(memory: Vec<i64>, input: Vec<i64>, center: (usize, usize)) -> Self {
        let mut memory_region = [0; 0x1000];
        for (i, data) in memory.into_iter().enumerate() {
            memory_region[i] = data;
        }

        let mut state = MachineState::default();
        for x in input {
            state.push_input(x);
        }

        HullPainter {
            memory_region: memory_region.to_vec(),
            state,
            position: center,
            direction: Up,
        }
    }
}

impl HullPainter {
    pub fn done(&self) -> bool {
        self.state.halted()
    }

    pub fn round(&mut self, canvas: &mut [Color], dimensions: &(usize, usize)) {
        self.paint(canvas, dimensions);
        self.mov();

        if let Color::White = self.get_under(canvas, dimensions) {
            self.state.push_input(1);
        } else {
            self.state.push_input(0);
        }
    }

    fn get_under(&self, canvas: &mut [Color], dimensions: &(usize, usize)) -> Color {
        canvas[self.position.1 * dimensions.0 + self.position.0]
    }

    fn paint(&mut self, canvas: &mut [Color], dimensions: &(usize, usize)) {
        // Compute painting color
        self.run_until_output();
        if self.done() {
            return;
        }

        let color = match self.state.output_mut().pop_front().unwrap() {
            0 => Color::Black,
            1 => Color::White,
            _ => panic!("Invalid color code !")
        };

        // Paint
        let pos = self.position.1 * dimensions.0 + self.position.0;
        canvas[pos] = color;
    }

    fn mov(&mut self) {
        // Rotate
        self.run_until_output();
        if self.done() {
            return;
        }

        match self.state.output_mut().pop_front().unwrap() {
            0 => self.direction = Direction::from((self.direction as isize - 1).rem_euclid(4)),
            1 => self.direction = Direction::from((self.direction as isize + 1).rem_euclid(4)),
            _ => panic!("Invalid direction !"),
        }

        // Move
        match self.direction {
            Up => self.position.1 -= 1,
            Right => self.position.0 += 1,
            Down => self.position.1 += 1,
            Left => self.position.0 -= 1,
        }
    }

    fn run_until_output(&mut self) {
        while self.state.output().is_empty() && !self.state.halted() {
            let instr = parser::parse_next_instr_to_runnable(
                &self.memory_region[self.state.ip()..],
                self.state.ip(),
                self.state.bp(),
            );
            instr.run(&mut self.memory_region, &mut self.state);
        }
    }
}

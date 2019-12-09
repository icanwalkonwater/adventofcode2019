use crate::param_iterator::ParamMode::{Immediate, Position, Relative};

#[derive(Debug)]
pub enum ParamMode {
    Position,
    Immediate,
    Relative,
}

pub struct ParamHolder {
    params: Vec<char>,
    current: usize,
}

impl ParamHolder {
    pub fn from(params: Vec<char>) -> ParamHolder {
        ParamHolder { params, current: 0 }
    }
}

impl Iterator for ParamHolder {
    type Item = ParamMode;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(&mode) = self.params.get(self.current) {
            self.current += 1;

            match mode {
                '0' => Some(Position),
                '1' => Some(Immediate),
                '2' => Some(Relative),
                _ => panic!("Unknown parameter mode !"),
            }
        } else {
            Some(ParamMode::Position)
        }
    }
}

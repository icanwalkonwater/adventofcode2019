#[derive(Debug)]
pub enum ParamMode {
    Position,
    Immediate,
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

            if mode == '1' {
                Some(ParamMode::Immediate)
            } else {
                Some(ParamMode::Position)
            }
        } else {
            Some(ParamMode::Position)
        }
    }
}

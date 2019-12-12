use crate::line::Orientation::{Horizontal, Vertical};
use crate::Point;

struct Line {
    segments: Vec<Segment>,
}

enum Orientation {
    Horizontal,
    Vertical,
}

struct Segment {
    orientation: Orientation,
    from: Point,
    to: Point,
}

impl Segment {
    //noinspection DuplicatedCode
    pub fn new(raw: &str, from: &Point) -> Segment {
        let direction = match raw.chars().nth(0).unwrap() {
            'U' => Point::new(0, 1),
            'D' => Point::new(0, -1),
            'L' => Point::new(-1, 0),
            'R' => Point::new(1, 0),
            _ => panic!("Unknown direction"),
        };

        let length: i32 = raw[1..].parse().expect("Invalid length");

        Segment {
            orientation: if direction.x() == 0 {
                Vertical
            } else {
                Horizontal
            },
            from: from.clone(),
            to: Point::new(
                from.x() + (direction.x() * length),
                from.y() + (direction.y() * length),
            ),
        }
        .normalize()
    }

    pub fn normalize(self) -> Self {
        if let Horizontal() = self.orientation {
            if self.from.y() > self.to.y() {
                Segment {
                    orientation: Horizontal,
                    from: self.to,
                    to: self.from,
                }
            } else {
                self
            }
        } else {
            // Vertical
            if self.from.x() > self.to.x() {
                Segment {
                    orientation: Vertical,
                    from: self.to,
                    to: self.from,
                }
            } else {
                self
            }
        }
    }

    pub fn intersect_with(&self, rhs: &Segment) -> bool {
        if let Horizontal() = self.orientation {
            if let Horizontal() = rhs.orientation {
                // H H
                // Need to be both on the same line
                self.from.y() == rhs.from.y()
            } else {
                // H V
                // Need for self.y to be in the range of rhs
                // Need for rhs.x to be in the range of self
                (rhs.from.y()..rhs.to.y()).contains(&self.from.y())
                    && (self.from.x()..self.to.x()).contains(&rhs.from.x())
            }
        } else {
            // Vertical
            if let Vertical() = rhs.orientation {
                // V V
                // Need to be both on the same column
                self.from.x() == rhs.from.x()
            } else {
                // V H
                // Basically the oposite of H V
                (self.from.y()..self.to.y()).contains(&rhs.from.y())
                    && (rhs.from.x()..rhs.to.x()).contains(&self.from.x())
            }
        }
    }

    pub fn intersections_with(&self, other: &Segment) -> Option<Vec<Point>> {
        if !self.intersect_with(other) {
            None
        } else {
            if self.orientation == other.orientation {
                // Great
                if let Horizontal() = self.orientation {
                    None
                } else {
                    None
                }
            } else {
                None
            }
        }
    }
}

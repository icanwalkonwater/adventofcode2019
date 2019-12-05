use crate::again::Orientation::{Horizontal, Vertical};
use crate::Point;
use std::slice::Iter;

#[derive(Debug)]
pub struct Node {
    me: Point,
    neigh: Vec<(Node, Segment)>,
}

impl Node {
    pub fn new(me: Point) -> Self {
        Node {
            me,
            neigh: vec![],
        }
    }

    pub fn neigh(&self) -> &[(Self, Segment)] {
        self.neigh.as_slice()
    }

    pub fn neigh_mut(&mut self) -> &mut [(Self, Segment)] {
        self.neigh.as_mut_slice()
    }

    pub fn iter_neigh(&self) -> Iter<'_, (Self, Segment)> {
        self.neigh.iter()
    }

    pub fn connect(&mut self, node: Self, neigh: Segment) {
        self.neigh.push((node, neigh));
    }

    pub fn split_at(lhs: &Self, rhs: &Self) -> Self {
        // TODO
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Clone, Debug)]
pub struct Segment<> {
    orientation: Orientation,
    from: Point,
    to: Point,
}

impl Segment {
    pub fn new(from: Point, to: Point) -> Segment {
        if from.y() == to.y() {
            // Horizontal
            if from.x() < to.x() {
                Segment {
                    orientation: Horizontal,
                    from,
                    to,
                }
            } else {
                Segment {
                    orientation: Horizontal,
                    from: to,
                    to: from,
                }
            }
        } else {
            // Vertical
            if from.y() < to.y() {
                Segment {
                    orientation: Vertical,
                    from,
                    to,
                }
            } else {
                Segment {
                    orientation: Vertical,
                    from: to,
                    to: from,
                }
            }
        }
    }

    pub fn len(&self) -> u32 {
        if let Horizontal = self.orientation {
            (self.to.x() - self.from.x()) as u32
        } else {
            (self.to.y() - self.from.y()) as u32
        }
    }

    pub fn intersection(&self, with: &Segment) -> Option<Point> {
        if !self.intersect_with(with) {
            None
        } else {
            if self.orientation == with.orientation {
                panic!("Found straight line !");
            }

            if let Horizontal = self.orientation {
                Some(Point::new(with.from.x(), self.from.y()))
            } else {
                Some(Point::new(self.from.x(), with.from.y()))
            }
        }
    }

    fn intersect_with(&self, rhs: &Segment) -> bool {
        if let Horizontal = self.orientation {
            if let Horizontal = rhs.orientation {
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
            if let Vertical = rhs.orientation {
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
}

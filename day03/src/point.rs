use std::cmp::Ordering;
use std::ops::Add;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn dist_from_center(&self) -> u32 {
        (self.x.abs() + self.y.abs()) as u32
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.dist_from_center()
            .partial_cmp(&other.dist_from_center())
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        self.dist_from_center().cmp(&other.dist_from_center())
    }
}

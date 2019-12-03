use crate::Point;

#[derive(Debug)]
pub struct Node {
    delta: Point,
    length: u32,
}

impl Node {
    pub fn new(raw: &str) -> Node {
        let delta = match raw.chars().nth(0).unwrap() {
            'U' => Point::new(0, 1),
            'D' => Point::new(0, -1),
            'L' => Point::new(-1, 0),
            'R' => Point::new(1, 0),
            _ => panic!("Unknown direction"),
        };

        let length: u32 = raw[1..].parse().expect("Invalid length");

        // Expand point
        Node { delta, length }
    }

    pub fn expand_from_into(self, from: Point) -> NodeExpander {
        NodeExpander {
            from,
            delta: self.delta,
            left: self.length,
        }
    }
}

pub struct NodeExpander {
    from: Point,
    delta: Point,
    left: u32,
}

impl Iterator for NodeExpander {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.left > 0 {
            let next = self.from.clone() + self.delta.clone();
            self.from = next.clone();
            self.left -= 1;

            Some(next)
        } else {
            None
        }
    }
}

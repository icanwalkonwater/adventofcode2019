use crate::Node;
use crate::Point;
use crate::Segment;

pub fn parse_line(line: Vec<&str>) -> Vec<(Point, i32)> {
    let line: Vec<(Point, i32)> = line
        .into_iter()
        .map(|mov| {
            let direction = match mov.chars().nth(0).unwrap() {
                'U' => Point::new(0, 1),
                'D' => Point::new(0, -1),
                'L' => Point::new(-1, 0),
                'R' => Point::new(1, 0),
                _ => panic!("Unknown direction"),
            };

            let len: i32 = mov[1..].parse().expect("Invalid length");

            (direction, len)
        })
        .collect();

    line
}

pub fn build_tree(line: Vec<(Point, i32)>) -> Node {
    let mut points = vec![Point::new(0, 0)];
    let mut segments = vec![];

    // Build points and segments
    for (direction, len) in line.into_iter() {
        let from = points.last().unwrap();

        let to = Point::new(
            from.x() + (direction.x() * len),
            from.y() + (direction.y() * len),
        );

        segments.push(Segment::new(from.clone(), to.clone()));
        points.push(to);
    }

    // Convert to nodes

    // Create root node
    let mut root_node = Node::new(points.first().unwrap().clone());
    // Temp storage of nodes
    let mut nodes: Vec<&mut Node> = Vec::with_capacity(points.len());
    nodes.push(&mut root_node);
    let mut last_node = &mut root_node;

    // Attach to previous node
    for (point, segment) in points.iter().skip(1).zip(segments.iter()) {
        // Create node
        let node = Node::new(point.clone());
        last_node.connect(node, segment.clone());

        last_node = &mut last_node.neigh_mut()[0].0;
    }

    root_node
}

pub fn split_at_intersection(root: Node) {
    root.iter_neigh()
}

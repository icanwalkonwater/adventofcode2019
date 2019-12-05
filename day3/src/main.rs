//use std::fs;

static DEMO: &str = "R8,U5,L5,D3\nU7,R6,D4,L4";
// static DEMO: &str = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
// static DEMO: &str = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7";

fn main() {
    //let lines = fs::read_to_string("./input/day3.txt")
    //    .expect("Please put your input under ./input/day3.txt");

    let lines = DEMO;

    let lines: Vec<Vec<&str>> = lines
        .trim()
        .lines()
        .map(|line| line.split(',').collect::<Vec<&str>>())
        .collect();

    let line1 = lines[0].clone();
    let _line2 = lines[1].clone();

    let parsed = day3::parse_line(line1);
    println!("Parsed: {:?}", parsed);

    let tree = day3::build_tree(parsed);
    println!("Tree: {:#?}", tree);

    /*let line1 = expand_line(line1);
    let line2 = expand_line(line2);

    let intersections = find_intersections(&line1, &line2);
    let min = intersections.into_iter().min().unwrap();

    println!("Day 3:");
    println!("-- Part 1: {}", min.dist_from_center());*/
}

/*
fn expand_line(line: &[&str]) -> Vec<Point> {
    // Parse line into nodes
    let nodes: Vec<Node> = line
        .into_iter()
        .map(|&instruction| Node::new(instruction))
        .collect();

    let mut points = vec![Point::new(0, 0)];

    // Expand nodes one after the other
    for node in nodes {
        let from = points.last().unwrap().clone();
        points.extend(node.expand_from_into(from));
    }

    // Remove {0, 0}
    points.remove(0);
    points
}

fn find_intersections<'a>(line1: &'a [Point], line2: &[Point]) -> Vec<&'a Point> {
    let mut intersections = Vec::new();

    'outer: for point in line1 {
        for item in line2 {
            if point == item {
                intersections.push(point);
                continue 'outer;
            }
        }
    }

    intersections
}
*/

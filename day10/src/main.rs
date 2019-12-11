use std::fs;

use day10::{extract_asteroids, find_asteroid_with_most_sight, start_laser};

#[cfg(test)]
mod tests {
    use day10::{consume_laser_round, extract_asteroids, find_asteroid_with_most_sight, Point, start_laser};

    #[test]
    fn test_example33() {
        let map = "......#.#.\n#..#.#....\n..#######.\n.#.#.###..\n.#..#.....\n..#....#.#\n#..#....#.\n.##.#..###\n##...#..#.\n.#....####"
            .lines()
            .into_iter()
            .map(|line| line.chars().map(|c| c == '#').collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let asteroids = extract_asteroids(&map);
        let (_, max_in_sight) = find_asteroid_with_most_sight(&map, &asteroids);

        assert_eq!(max_in_sight, 33);
    }

    #[test]
    fn test_example35() {
        let map = "#.#...#.#.\n.###....#.\n.#....#...\n##.#.#.#.#\n....#.#.#.\n.##..###.#\n..#...##..\n..##....##\n......#...\n.####.###."
            .lines()
            .into_iter()
            .map(|line| line.chars().map(|c| c == '#').collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let asteroids = extract_asteroids(&map);
        let (_, max_in_sight) = find_asteroid_with_most_sight(&map, &asteroids);

        assert_eq!(max_in_sight, 35);
    }

    #[test]
    fn test_example41() {
        let map = ".#..#..###\n####.###.#\n....###.#.\n..###.##.#\n##.##.#.#.\n....###..#\n..#.#..#.#\n#..#.#.###\n.##...##.#\n.....#.#.."
            .lines()
            .into_iter()
            .map(|line| line.chars().map(|c| c == '#').collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let asteroids = extract_asteroids(&map);
        let (_, max_in_sight) = find_asteroid_with_most_sight(&map, &asteroids);

        assert_eq!(max_in_sight, 41);
    }

    #[test]
    fn test_example210() {
        let map = ".#..##.###...#######\n##.############..##.\n.#.######.########.#\n.###.#######.####.#.\n#####.##.#.##.###.##\n..#####..#.#########\n####################\n#.####....###.#.#.##\n##.#################\n#####.##.###..####..\n..######..##.#######\n####.##.####...##..#\n.#####..#.######.###\n##...#.##########...\n#.##########.#######\n.####.#.###.###.#.##\n....##.##.###..#####\n.#.#.###########.###\n#.#.#.#####.####.###\n###.##.####.##.#..##"
            .lines()
            .into_iter()
            .map(|line| line.chars().map(|c| c == '#').collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let asteroids = extract_asteroids(&map);
        let (_, max_in_sight) = find_asteroid_with_most_sight(&map, &asteroids);

        assert_eq!(max_in_sight, 210);
    }

    #[test]
    fn test_example_laser() {
        let mut map = ".#....#####...#..\n##...##.#####..##\n##...#...#.#####.\n..#.....#...###..\n..#.#.....#....##"
            .lines()
            .into_iter()
            .map(|line| line.chars().map(|c| c == '#').collect::<Vec<_>>())
            .collect::<Vec<_>>();

        // First rotation
        let asteroids = extract_asteroids(&map);
        let vaporized = consume_laser_round(&mut map, asteroids, &Point::new(8, 3));
        let mut it = vaporized.iter();

        assert_eq!(it.next(), Some(&Point::new(8, 1))); // 1
        assert_eq!(it.next(), Some(&Point::new(9, 0))); // 2
        assert_eq!(it.next(), Some(&Point::new(9, 1))); // 3
        assert_eq!(it.next(), Some(&Point::new(10, 0))); // 4
        assert_eq!(it.next(), Some(&Point::new(9, 2))); // 5
        assert_eq!(it.next(), Some(&Point::new(11, 1))); // 6
        assert_eq!(it.next(), Some(&Point::new(12, 1))); // 7
        assert_eq!(it.next(), Some(&Point::new(11, 2))); // 8
        assert_eq!(it.next(), Some(&Point::new(15, 1))); // 9

        assert_eq!(it.next(), Some(&Point::new(12, 2))); // 1
        assert_eq!(it.next(), Some(&Point::new(13, 2))); // 2
        assert_eq!(it.next(), Some(&Point::new(14, 2))); // 3
        assert_eq!(it.next(), Some(&Point::new(15, 2))); // 4
        assert_eq!(it.next(), Some(&Point::new(12, 3))); // 5
        assert_eq!(it.next(), Some(&Point::new(16, 4))); // 6
        assert_eq!(it.next(), Some(&Point::new(15, 4))); // 7
        assert_eq!(it.next(), Some(&Point::new(10, 4))); // 8
        assert_eq!(it.next(), Some(&Point::new(4, 4))); // 9

        assert_eq!(it.next(), Some(&Point::new(2, 4))); // 1
        assert_eq!(it.next(), Some(&Point::new(2, 3))); // 2
        assert_eq!(it.next(), Some(&Point::new(0, 2))); // 3
        assert_eq!(it.next(), Some(&Point::new(1, 2))); // 4
        assert_eq!(it.next(), Some(&Point::new(0, 1))); // 5
        assert_eq!(it.next(), Some(&Point::new(1, 1))); // 6
        assert_eq!(it.next(), Some(&Point::new(5, 2))); // 7
        assert_eq!(it.next(), Some(&Point::new(1, 0))); // 8
        assert_eq!(it.next(), Some(&Point::new(5, 1))); // 9

        assert_eq!(it.next(), Some(&Point::new(6, 1))); // 1
        assert_eq!(it.next(), Some(&Point::new(6, 0))); // 2
        assert_eq!(it.next(), Some(&Point::new(7, 0))); // 3
        assert_eq!(it.next(), None);

        // Second rotation
        let asteroids = extract_asteroids(&map);
        let vaporized = consume_laser_round(&mut map, asteroids, &Point::new(8, 3));
        let mut it = vaporized.iter();

        assert_eq!(it.next(), Some(&Point::new(8, 0))); // 4
        assert_eq!(it.next(), Some(&Point::new(10, 1))); // 5
        assert_eq!(it.next(), Some(&Point::new(14, 0))); // 6
        assert_eq!(it.next(), Some(&Point::new(16, 1))); // 7
        assert_eq!(it.next(), Some(&Point::new(13, 3))); // 8
        assert_eq!(it.next(), None);

        // Third rotation
        let asteroids = extract_asteroids(&map);
        let vaporized = consume_laser_round(&mut map, asteroids, &Point::new(8, 3));
        let mut it = vaporized.iter();

        assert_eq!(it.next(), Some(&Point::new(14, 3))); // 9
        assert_eq!(it.next(), None);

        // Everything is destroyed
        let asteroids = extract_asteroids(&map);
        assert_eq!(asteroids.len(), 1);

        let vaporized = consume_laser_round(&mut map, asteroids, &Point::new(8, 3));
        assert!(vaporized.is_empty());
    }

    #[test]
    fn test_large_laser() {
        let mut map = ".#..##.###...#######\n##.############..##.\n.#.######.########.#\n.###.#######.####.#.\n#####.##.#.##.###.##\n..#####..#.#########\n####################\n#.####....###.#.#.##\n##.#################\n#####.##.###..####..\n..######..##.#######\n####.##.####...##..#\n.#####..#.######.###\n##...#.##########...\n#.##########.#######\n.####.#.###.###.#.##\n....##.##.###..#####\n.#.#.###########.###\n#.#.#.#####.####.###\n###.##.####.##.#..##"
            .lines()
            .into_iter()
            .map(|line| line.chars().map(|c| c == '#').collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let asteroids = extract_asteroids(&map);
        let (center, _) = find_asteroid_with_most_sight(&map, &asteroids);

        let destroy_order = start_laser(&mut map, center);

        assert!(destroy_order.len() > 200);
        assert_eq!(destroy_order[199], Point::new(8, 2)); // 200th
    }
}

fn main() {
    let mut map = fs::read_to_string("./input/day10.txt")
        .expect("Please put your input under ./input/day10.txt")
        .lines()
        .into_iter()
        .map(|line| line.chars().map(|c| c == '#').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let asteroids = extract_asteroids(&map);
    let (center, in_sight) = find_asteroid_with_most_sight(&map, &asteroids);

    let destroy_order = start_laser(&mut map, center);
    let asteroid_200th = &destroy_order[199];

    println!("Day 10:");
    println!("-- Part 1: {}", in_sight);
    println!("-- Part 2: {}", asteroid_200th.x() * 100 + asteroid_200th.y());
}

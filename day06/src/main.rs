use std::fs;

use day6::{build_system, count_orbits, count_transits};

#[cfg(test)]
mod tests {
    use day6::{build_system, count_orbits, count_transits};

    #[test]
    fn test_example_part1() {
        let (system, _) = build_system(vec![
            ("COM", "B"),
            ("B", "C"),
            ("C", "D"),
            ("D", "E"),
            ("E", "F"),
            ("B", "G"),
            ("G", "H"),
            ("D", "I"),
            ("E", "J"),
            ("J", "K"),
            ("K", "L"),
        ]);

        assert_eq!(count_orbits(&system), 42);
    }

    #[test]
    fn test_example_part2() {
        let solar_map = vec![
            ("COM", "B"),
            ("B", "C"),
            ("C", "D"),
            ("D", "E"),
            ("E", "F"),
            ("B", "G"),
            ("G", "H"),
            ("D", "I"),
            ("E", "J"),
            ("J", "K"),
            ("K", "L"),
            ("K", "YOU"),
            ("I", "SAN"),
        ];

        let (system, orbits) = build_system(solar_map);

        assert_eq!(count_orbits(&system), 54);
        assert_eq!(count_transits(&orbits, "YOU", "SAN"), 4);
    }
}

fn main() {
    let solar_map = fs::read_to_string("./input/day6.txt")
        .expect("Please put your input under ./input/day6.txt");

    let solar_map: Vec<(&str, &str)> = solar_map
        .lines()
        .map(|relation| {
            let (planet, moon) = relation.split_at(relation.find(')').unwrap());
            (planet, &moon[1..])
        })
        .collect();

    let (system, orbits) = build_system(solar_map);

    let orbits_count = count_orbits(&system);
    let transit_count = count_transits(&orbits, "YOU", "SAN");

    println!("Day 6:");
    println!("-- Part 1: {}", orbits_count);
    println!("-- Part 2: {}", transit_count);
}

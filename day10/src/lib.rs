pub use laser::consume_laser_round;
pub use laser::sort_asteroids_by_angle;
pub use laser::start_laser;
pub use point::Point;
pub use vision::count_asteroid_on_sight;
pub use vision::find_asteroid_with_most_sight;
pub use vision::get_asteroids_on_sight;
pub use vision::is_on_sight;

mod laser;
mod point;
mod vision;

pub fn extract_asteroids(map: &Vec<Vec<bool>>) -> Vec<Point> {
    let mut asteroids = Vec::new();

    for (y, line) in map.iter().enumerate() {
        for (x, &is_present) in line.iter().enumerate() {
            if is_present {
                asteroids.push(Point::new(x, y));
            }
        }
    }

    asteroids
}

#[cfg(test)]
mod tests {
    use crate::{count_asteroid_on_sight, extract_asteroids, is_on_sight, Point};

    fn create_map() -> Vec<Vec<bool>> {
        vec![
            vec![
                true, false, false, false, false, false, false, false, false, false,
            ],
            vec![
                false, false, false, true, false, false, false, false, false, false,
            ],
            vec![
                false, false, false, true, false, false, false, false, false, false,
            ],
            vec![
                false, true, true, true, true, false, false, false, false, false,
            ],
            vec![
                false, false, true, false, false, false, false, false, false, false,
            ],
            vec![
                false, false, false, false, false, false, false, false, false, false,
            ],
            vec![
                false, false, false, false, false, false, false, false, false, false,
            ],
            vec![
                false, false, false, false, false, false, false, false, false, false,
            ],
            vec![
                false, false, false, false, false, false, false, false, false, false,
            ],
            vec![
                false, false, false, false, false, false, false, false, false, false,
            ],
        ]
    }

    fn create_map_small() -> Vec<Vec<bool>> {
        vec![
            vec![false, true, false, false, true],
            vec![false, false, false, false, false],
            vec![true, true, true, true, true],
            vec![false, false, false, false, true],
            vec![false, false, false, true, true],
        ]
    }

    #[test]
    fn test_on_sight() {
        let map = create_map();

        assert!(is_on_sight(&map, &Point::new(0, 0), &Point::new(3, 1)));
        assert!(!is_on_sight(&map, &Point::new(0, 0), &Point::new(9, 3)));

        assert!(is_on_sight(&map, &Point::new(0, 0), &Point::new(3, 2)));
        assert!(!is_on_sight(&map, &Point::new(0, 0), &Point::new(6, 4)));

        assert!(is_on_sight(&map, &Point::new(0, 0), &Point::new(4, 2)));

        assert!(is_on_sight(&map, &Point::new(1, 3), &Point::new(2, 3)));
        assert!(!is_on_sight(&map, &Point::new(1, 3), &Point::new(3, 3)));
    }

    #[test]
    fn test_extract_asteroids() {
        let map = create_map();

        let asteroids = extract_asteroids(&map);

        assert_eq!(asteroids.len(), 8);
        assert!(asteroids.contains(&Point::new(0, 0)));
        assert!(asteroids.contains(&Point::new(3, 2)));
        assert!(asteroids.contains(&Point::new(3, 3)));
        assert!(asteroids.contains(&Point::new(1, 3)));
        assert!(asteroids.contains(&Point::new(2, 3)));
        assert!(asteroids.contains(&Point::new(3, 3)));
        assert!(asteroids.contains(&Point::new(4, 3)));

        assert!(!asteroids.contains(&Point::new(9, 9)));
        assert!(!asteroids.contains(&Point::new(3, 4)));
        assert!(!asteroids.contains(&Point::new(3, 5)));
    }

    #[test]
    fn test_count_in_sight() {
        let map = create_map_small();
        let asteroids = extract_asteroids(&map);

        assert_eq!(asteroids.len(), 10);

        assert!(is_on_sight(&map, &Point::new(1, 0), &Point::new(4, 0)));
        assert!(is_on_sight(&map, &Point::new(1, 0), &Point::new(0, 2)));
        assert!(is_on_sight(&map, &Point::new(1, 0), &Point::new(1, 2)));
        assert!(is_on_sight(&map, &Point::new(1, 0), &Point::new(2, 2)));
        assert!(is_on_sight(&map, &Point::new(1, 0), &Point::new(3, 2)));
        assert!(is_on_sight(&map, &Point::new(1, 0), &Point::new(4, 2)));
        assert!(!is_on_sight(&map, &Point::new(1, 0), &Point::new(4, 3)));
        assert!(!is_on_sight(&map, &Point::new(1, 0), &Point::new(3, 4)));

        assert_eq!(
            count_asteroid_on_sight(&map, &asteroids, &Point::new(1, 0)),
            7
        );
        assert_eq!(
            count_asteroid_on_sight(&map, &asteroids, &Point::new(4, 0)),
            7
        );
        assert_eq!(
            count_asteroid_on_sight(&map, &asteroids, &Point::new(0, 2)),
            6
        );
        assert_eq!(
            count_asteroid_on_sight(&map, &asteroids, &Point::new(1, 2)),
            7
        );
        assert_eq!(
            count_asteroid_on_sight(&map, &asteroids, &Point::new(2, 2)),
            7
        );
        assert_eq!(
            count_asteroid_on_sight(&map, &asteroids, &Point::new(3, 2)),
            7
        );
        assert_eq!(
            count_asteroid_on_sight(&map, &asteroids, &Point::new(4, 2)),
            5
        );
        assert_eq!(
            count_asteroid_on_sight(&map, &asteroids, &Point::new(4, 3)),
            7
        );
        assert_eq!(
            count_asteroid_on_sight(&map, &asteroids, &Point::new(4, 4)),
            7
        );
        assert_eq!(
            count_asteroid_on_sight(&map, &asteroids, &Point::new(3, 4)),
            8
        );
    }
}

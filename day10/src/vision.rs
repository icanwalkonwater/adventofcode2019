use num_integer;

use crate::Point;

pub fn find_asteroid_with_most_sight<'a>(
    map: &Vec<Vec<bool>>,
    asteroids: &'a Vec<Point>,
) -> (&'a Point, usize) {
    asteroids
        .iter()
        .map(|asteroid| {
            let count = count_asteroid_on_sight(&map, &asteroids, asteroid);
            (asteroid, count)
        })
        .max_by(|(_, count1), (_, count2)| count1.cmp(count2))
        .unwrap()
}

pub fn count_asteroid_on_sight(
    map: &Vec<Vec<bool>>,
    asteroids: &Vec<Point>,
    from: &Point,
) -> usize {
    asteroids
        .iter()
        .filter(|&asteroid| asteroid != from)
        .filter(|&asteroid| is_on_sight(map, from, asteroid))
        .count()
}

pub fn get_asteroids_on_sight(
    map: &Vec<Vec<bool>>,
    asteroids: &Vec<Point>,
    center: &Point,
) -> Vec<Point> {
    asteroids
        .iter()
        .filter(|&asteroid| asteroid != center)
        .filter(|&asteroid| is_on_sight(map, center, asteroid))
        .map(|asteroid| asteroid.clone())
        .collect::<Vec<_>>()
}

pub fn is_on_sight(map: &Vec<Vec<bool>>, from: &Point, to: &Point) -> bool {
    let diff_x = to.x() as isize - from.x() as isize;
    let diff_y = to.y() as isize - from.y() as isize;
    let step_amount = num_integer::gcd(diff_x, diff_y) as usize;

    if diff_x == 0 {
        let x = from.x();
        !(from.y().min(to.y())..from.y().max(to.y()))
            .skip(1) // Skip the one where the from cell is
            .any(|y| map[y][x])
    } else if diff_y == 0 {
        let y = from.y();
        !(from.x().min(to.x())..from.x().max(to.x()))
            .skip(1) // Skip the one where the from cell is
            .any(|x| map[y][x])
    } else if step_amount == 1 {
        true
    } else {
        let range_x = (from.x().min(to.x())..from.x().max(to.x()))
            .step_by(diff_x.abs() as usize / step_amount)
            .skip(1);

        let range_y = (from.y().min(to.y())..from.y().max(to.y()))
            .step_by(diff_y.abs() as usize / step_amount)
            .skip(1);

        if diff_x.is_positive() == diff_y.is_positive() {
            // If both deltas are the same sign, everything is fine, nothing was reversed alone
            !range_x.zip(range_y).any(|(x, y)| map[y][x])
        } else {
            // Otherwise, we need to reverse one of the range because one was reversed when created
            !range_x.rev().zip(range_y).any(|(x, y)| map[y][x])
        }
    }
}

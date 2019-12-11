use std::cmp::Ordering::Equal;
use std::f64::consts::PI;

use crate::{extract_asteroids, get_asteroids_on_sight, Point};

pub fn start_laser(map: &mut Vec<Vec<bool>>, center: &Point) -> Vec<Point> {
    let mut destroy_order = Vec::new();

    loop {
        let asteroids = extract_asteroids(map);
        let mut vaporized = consume_laser_round(map, asteroids, center);

        if vaporized.is_empty() {
            break;
        }

        destroy_order.append(&mut vaporized);
    }

    destroy_order
}

pub fn consume_laser_round(map: &mut Vec<Vec<bool>>, asteroids: Vec<Point>, center: &Point) -> Vec<Point> {
    let mut on_sight = sort_asteroids_by_angle(get_asteroids_on_sight(map, &asteroids, center), center);

    // Remove them from the map
    for asteroid in on_sight.iter() {
        map[asteroid.y()][asteroid.x()] = false;
    }

    // The asteroids will be consumed after that

    on_sight
}

pub fn sort_asteroids_by_angle(asteroids: Vec<Point>, center: &Point) -> Vec<Point> {
    let mut asteroids = asteroids
        .into_iter()
        .map(|asteroid| {
            let angle = compute_angle_between(center, &asteroid);
            (asteroid, angle)
        })
        .collect::<Vec<_>>();

    asteroids.sort_by(|(_, angle1), (_, angle2)| angle1.partial_cmp(angle2).unwrap_or(Equal));

    asteroids
        .into_iter()
        .map(|(asteroid, _)| asteroid)
        .collect()
}

fn compute_angle_between(center: &Point, target: &Point) -> f64 {
    let effective_cos = center.x() as f64 - target.x() as f64;
    let effective_sin = center.y() as f64 - target.y() as f64;

    (-effective_cos.atan2(effective_sin)).rem_euclid(PI + PI)
}

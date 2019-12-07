use std::collections::HashMap;

pub fn build_system<'a>(
    solar_map: Vec<(&'a str, &'a str)>,
) -> (HashMap<&'a str, Vec<&'a str>>, HashMap<&'a str, &'a str>) {
    // Contains the planets associated with their moons
    let mut moons = HashMap::new();
    // Contains the planets associated with their sun
    let mut orbits = HashMap::new();

    for (planet, moon) in solar_map {
        let planet_moons = moons.entry(planet).or_insert(Vec::new());
        planet_moons.push(moon);

        orbits.insert(moon, planet);
    }

    (moons, orbits)
}

pub fn count_orbits(system: &HashMap<&str, Vec<&str>>) -> u32 {
    __count_orbits(system, "COM", 0)
}

fn __count_orbits(system: &HashMap<&str, Vec<&str>>, name: &str, depth: u32) -> u32 {
    if let Some(moons) = system.get(name) {
        // If has moons
        let sum: u32 = moons
            .iter()
            .map(|moon| __count_orbits(system, moon, depth + 1))
            .sum();

        sum + depth
    } else {
        // Otherwise is a leaf
        depth
    }
}

pub fn count_transits<'a>(
    orbits: &'a HashMap<&'a str, &'a str>,
    from: &'a str,
    to: &'a str,
) -> usize {
    let path_to_com = |from| {
        let mut path = Vec::new();

        let mut current = from;
        while let Some(&sun) = orbits.get(current) {
            path.push(sun);
            current = sun;
        }

        path
    };

    let mut from_com_path = path_to_com(from);
    let mut to_com_path = path_to_com(to);

    from_com_path.reverse(); // COM -> from
    to_com_path.reverse(); // COM -> to

    let mut deeper_common_orbit_index = 0;
    {
        let mut from_it = from_com_path.iter().skip(1);
        let mut to_it = to_com_path.iter().skip(1);

        while from_it.next().unwrap() == to_it.next().unwrap() {
            deeper_common_orbit_index += 1;
        }
    }

    from_com_path.drain(..deeper_common_orbit_index + 1);
    to_com_path.drain(..deeper_common_orbit_index + 1);

    from_com_path.len() + to_com_path.len()
}

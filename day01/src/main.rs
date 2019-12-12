use std::fs;

fn main() {
    let input = fs::read_to_string("./input/day1.txt")
        .expect("Please put your input under ./input/day1.txt");

    let input: Vec<u32> = input
        .lines()
        .into_iter()
        .map(|line| line.parse::<u32>().unwrap())
        .collect();

    // Part 1
    let fuel1: u32 = input.iter().map(|mass| mass / 3 - 2).sum();

    // Part 2
    let fuel2: u32 = input
        .into_iter()
        .map(|mass| calculate_fuel_for_module(mass))
        .sum();

    println!("Day 1:");
    println!("-- Part 1: {}", fuel1);
    println!("-- Part 2: {}", fuel2);
}

fn calculate_fuel_for_module(mass: u32) -> u32 {
    let mut fuel_total = mass / 3 - 2;
    let mut fuel_needed = fuel_total as i32;

    loop {
        fuel_needed = fuel_needed / 3 - 2;
        if fuel_needed <= 0 {
            break;
        } else {
            fuel_total += fuel_needed as u32;
        }
    }

    fuel_total
}

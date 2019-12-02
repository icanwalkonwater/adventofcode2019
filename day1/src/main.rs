use std::fs;

fn main() {
    let input = fs::read_to_string("./input/day1.txt")
        .expect("Please put your input under ./input/day1.txt");

    let input: Vec<f64> = input
        .lines()
        .into_iter()
        .map(|line| line.parse::<f64>().unwrap())
        .collect();

    let result: u32 = input
        .into_iter()
        .map(|mass| mass / 3.)
        .map(|mass| mass as u32)
        .map(|mass| mass - 2)
        .sum();

    println!("{}", result);
}

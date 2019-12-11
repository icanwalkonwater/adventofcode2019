use std::fs;

use colored::Colorize;

use day11::{Color, start_hull_painter, start_hull_painter2};

fn main() {
    let memory = fs::read_to_string("./input/day11.txt")
        .expect("Please put your input under ./input/day11.txt")
        .trim()
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    /*let memory = vec![
        3, 100,
        104, 1,
        104, 0,
        3, 100,
        104, 0,
        104, 0,
        3, 100,
        104, 1,
        104, 0,
        3, 100,
        104, 0,
        104, 0,
        3, 100,
        104, 0,
        104, 1,
        99,
    ];*/

    let canvas = start_hull_painter(memory.clone());

    let painted = canvas.iter()
        .filter(|&&color| color != Color::None)
        .count();

    let canvas = start_hull_painter2(memory);

    println!("Day 11:");
    println!("-- Part 1: {}", painted);

    canvas.iter()
        .map(|color| {
            match color {
                Color::White => " ".on_white(),
                Color::Black => " ".on_magenta(),
                Color::None => " ".on_black(),
            }
        })
        .collect::<Vec<_>>()
        .chunks_exact(101)
        .for_each(|line| {
            for cell in line {
                print!("{}", cell);
            }
            println!();
        });
}

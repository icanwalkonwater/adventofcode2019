use std::fs;

use colored::Colorize;

use day8::{combine_layers, img_checksum};

#[cfg(test)]
mod tests {
    use day8::combine_layers;

    #[test]
    fn test_combine_2x2() {
        let pix_matrix = vec![
            vec![0, 2, 2, 2],
            vec![1, 1, 2, 2],
            vec![2, 2, 1, 2],
            vec![0, 0, 0, 0],
        ];

        let combined = combine_layers(&pix_matrix, 4);

        assert_eq!(combined, vec![0, 1, 1, 0]);
    }
}

fn main() {
    let width = 25;
    let height = 6;
    let layer_len = width * height;

    let pix_matrix = fs::read_to_string("./input/day8.txt")
        .expect("Please put your input under ./input/day8.txt")
        .chars()
        .collect::<Vec<char>>()
        .chunks_exact(layer_len)
        .map(|layer| Vec::from(layer))
        .map(|layer| {
            layer
                .iter()
                .map(|color| color.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let checksum = img_checksum(&pix_matrix);
    let combined = combine_layers(&pix_matrix, layer_len);

    println!("Day 8:");
    println!("-- Part 1: {}", checksum);

    println!("-- Part 2:");
    combined
        .iter()
        .map(|color| match color {
            0 => " ".on_magenta(),
            1 => " ".on_white(),
            _ => " ".on_black(),
        })
        .collect::<Vec<_>>()
        .chunks_exact(width)
        .for_each(|line| {
            for cell in line {
                print!("{}", cell);
            }
            println!();
        });
}

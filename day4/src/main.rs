use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_111111() {
        assert!(is_number_eligible("111111"));
    }

    #[test]
    fn test_223450() {
        assert!(!is_number_eligible("223450"));
    }

    #[test]
    fn test_123789() {
        assert!(!is_number_eligible("123789"));
    }

    #[test]
    fn test2_112233() {
        assert!(is_number_eligible("112233") && num_at_most_doubles("112233"));
    }

    #[test]
    fn test2_123444() {
        assert!(!(is_number_eligible("123444") && num_at_most_doubles("123444")));
    }

    #[test]
    fn test2_111122() {
        assert!(is_number_eligible("111122") && num_at_most_doubles("111122"));
    }
}

fn main() {
    let slices: Vec<u32> = fs::read_to_string("./input/day4.txt")
        .expect("Please put your input under ./input/day4.txt")
        .trim()
        .split('-')
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    let min = slices[0];
    let max = slices[1];

    let mut amount = 0;
    let mut amount_strong = 0;
    for i in min..=max {
        let as_str = i.to_string();

        if is_number_eligible(&as_str) {
            amount += 1;

            if num_at_most_doubles(&as_str) {
                amount_strong += 1;
            }
        }
    }

    println!("Day 4:");
    println!("-- Part 1: {}", amount);
    println!("-- Part 2: {}", amount_strong);
}

fn is_number_eligible(num: &str) -> bool {
    num_has_dup(num) && num_is_decreasing(num)
}

fn num_has_dup(num: &str) -> bool {
    let mut prev = num.chars().nth(0).unwrap();
    for item in num.chars().skip(1) {
        if prev == item {
            return true;
        }
        prev = item;
    }

    false
}

fn num_at_most_doubles(num: &str) -> bool {
    let mut occ = 1;
    let mut prev = num.chars().nth(0).unwrap();

    for item in num.chars().skip(1) {
        if prev == item {
            occ += 1;
        } else if occ == 2 {
            // Early return
            return true;
        } else {
            occ = 1;
        }

        prev = item;
    }

    // Final check
    occ == 2
}

fn num_is_decreasing(num: &str) -> bool {
    let mut prev = num.chars().nth(0).unwrap();
    for item in num.chars().skip(1) {
        if item < prev {
            return false;
        }
        prev = item;
    }

    true
}

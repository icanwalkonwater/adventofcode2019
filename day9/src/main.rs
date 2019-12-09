use day9::start_int_machine;
use std::fs;

#[cfg(test)]
mod tests {
    use day9::start_int_machine;

    #[test]
    fn test_sanity_check() {
        let memory = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];

        // < 8
        assert_eq!(start_int_machine(memory.clone(), vec![6]).output()[0], 999);
        // == 8
        assert_eq!(start_int_machine(memory.clone(), vec![8]).output()[0], 1000);
        // > 10
        assert_eq!(start_int_machine(memory, vec![10]).output()[0], 1001);
    }

    #[test]
    fn test_copy_itself() {
        let memory = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];

        let final_state = start_int_machine(memory.clone(), vec![]);
        assert_eq!(final_state.into_output(), memory);
    }

    #[test]
    fn test_16_digits() {
        let memory = vec![1102, 34_915_192, 34_915_192, 7, 4, 7, 99, 0];

        let final_state = start_int_machine(memory, vec![]);
        assert_eq!(final_state.output()[0].to_string().len(), 16);
    }

    #[test]
    fn test_large_number() {
        let memory = vec![104, 1_125_899_906_842_624, 99];

        let final_state = start_int_machine(memory, vec![]);
        assert_eq!(final_state.output()[0], 1_125_899_906_842_624);
    }
}

fn main() {
    let memory = fs::read_to_string("./input/day9.txt")
        .expect("Please put your input under ./input/day9.txt")
        .trim()
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let boost_check = start_int_machine(memory.clone(), vec![1]);
    let boost_distress = start_int_machine(memory, vec![2]);

    println!("Day 9:");
    println!("-- Part 1: {}", boost_check.output()[0]);
    println!("-- Part 2: {}", boost_distress.output()[0]);
}

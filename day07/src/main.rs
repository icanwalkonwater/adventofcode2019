use std::fs;

use day7::{compute_best_phase_cfg, compute_best_phase_cfg_lo};

#[cfg(test)]
mod tests {
    use day7::{compute_best_phase_cfg, compute_best_phase_cfg_lo};

    #[test]
    fn test_example_43210() {
        let memory = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];

        let (phases, signal) = compute_best_phase_cfg(memory);

        assert_eq!(phases, vec![4, 3, 2, 1, 0]);
        assert_eq!(signal, 43210);
    }

    #[test]
    fn test_example_54321() {
        let memory = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];

        let (phases, signal) = compute_best_phase_cfg(memory);

        assert_eq!(phases, vec![0, 1, 2, 3, 4]);
        assert_eq!(signal, 54321);
    }

    #[test]
    fn test_example_65210() {
        let memory = vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];

        let (phases, signal) = compute_best_phase_cfg(memory);

        assert_eq!(phases, vec![1, 0, 4, 3, 2]);
        assert_eq!(signal, 65210);
    }

    #[test]
    fn test_example_lo_139629729() {
        let memory = vec![
            3, 26, // 0: *26 = read() {phase}
            1001, 26, -4, 26, // 2: *26{phase - 4} += -4
            3, 27, // 6: *27 = read() {signal}
            1002, 27, 2, 27, // 8: *27{signal + 2} += 2
            1, 27, 26, 27, // 12: *27{signal + phase - 2} += *26{phase - 4}
            4, 27, // 16: write(*27) {signal + phase - 2}
            1001, 28, -1, 28, // 18: *28 += -1
            1005, 28, 6,  // 22: if *28 -> 6
            99, // 25: HALT
            0, 0, 5,
            // 26, 27, 28: data
        ];

        let (phases, signal) = compute_best_phase_cfg_lo(memory);

        assert_eq!(phases, vec![9, 8, 7, 6, 5]);
        assert_eq!(signal, 139_629_729);
    }

    #[test]
    fn test_example_lo_18216() {
        let memory = vec![
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ];

        let (phases, signal) = compute_best_phase_cfg_lo(memory);

        assert_eq!(phases, vec![9, 7, 8, 5, 6]);
        assert_eq!(signal, 18216);
    }
}

fn main() {
    // Load and parse the file to create the initial memory
    let memory: Vec<i32> = fs::read_to_string("./input/day7.txt")
        .expect("Please put your input under ./input/day7.txt")
        .trim()
        .split(',')
        .map(|num| num.parse::<i32>().unwrap())
        .collect();

    let (_, signal1) = compute_best_phase_cfg(memory.clone());
    let (_, signal2) = compute_best_phase_cfg_lo(memory);

    println!("Day 7:");
    println!("-- Part 1: {}", signal1);
    println!("-- Part 2: {}", signal2);
}

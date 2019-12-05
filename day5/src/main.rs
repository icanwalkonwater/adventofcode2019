use std::fs;

#[cfg(test)]
mod tests {
    use day5::run;

    #[test]
    fn test_eq_8_pos() {
        let memory = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];

        assert_eq!(run(memory.clone(), vec![8]), 1);
        assert_eq!(run(memory, vec![7]), 0);
    }

    #[test]
    fn test_eq_8_imm() {
        let memory = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];

        assert_eq!(run(memory.clone(), vec![8]), 1);
        assert_eq!(run(memory, vec![7]), 0);
    }

    #[test]
    fn test_lt_8_pos() {
        let memory = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];

        assert_eq!(run(memory.clone(), vec![7]), 1);
        assert_eq!(run(memory, vec![9]), 0);
    }

    #[test]
    fn test_lt_8_imm() {
        let memory = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];

        assert_eq!(run(memory.clone(), vec![7]), 1);
        assert_eq!(run(memory, vec![9]), 0);
    }

    #[test]
    fn test_jmp_pos() {
        let memory = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];

        assert_eq!(run(memory.clone(), vec![0]), 0);
        assert_eq!(run(memory, vec![5]), 1);
    }

    #[test]
    fn test_jmp_imm() {
        let memory = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        // 0: read(*3)
        // 2: jmp true(-1{input}, 9)
        //      -- true
        // 9:   write(*12{1})
        // 11:  HALT
        //      -- false
        // 5:   add(0, 0, *12) // *12 = 0
        // 9:   write(*12{0})
        // 11:  HALT

        assert_eq!(run(memory.clone(), vec![0]), 0);
        assert_eq!(run(memory, vec![5]), 1);
    }

    #[test]
    fn test_large() {
        let memory = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];

        // < 8
        assert_eq!(run(memory.clone(), vec![6]), 999);
        // == 8
        assert_eq!(run(memory.clone(), vec![8]), 1000);
        // > 10
        assert_eq!(run(memory, vec![10]), 1001);
    }
}

fn main() {
    // Load and parse the file to create the initial memory
    let memory: Vec<i32> = fs::read_to_string("./input/day5.txt")
        .expect("Please put your input under ./input/day5.txt")
        .trim()
        .split(',')
        .map(|num| num.parse::<i32>().unwrap())
        .collect();

    let output1 = day5::run(memory.clone(), vec![1]);
    let output2 = day5::run(memory, vec![5]);

    println!("Day 5:");
    println!("-- Part 1: {}", output1);
    println!("-- Part 2: {}", output2);
}
